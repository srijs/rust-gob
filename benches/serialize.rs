#[macro_use]
extern crate bencher;
extern crate bytes;
extern crate gob;
#[macro_use]
extern crate serde_derive;
extern crate serde_schema;
#[macro_use]
extern crate serde_schema_derive;

use bencher::Bencher;
use bytes::Buf;
use gob::StreamSerializer;
use serde_schema::SchemaSerialize;

#[derive(Serialize, SchemaSerialize)]
#[serde(rename = "Response")]
struct RpcResponse {
    #[serde(rename = "ServiceMethod")]
    service_method: &'static str,
    #[serde(rename = "Seq")]
    seq: u64,
    #[serde(rename = "Error", skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

#[derive(Serialize, SchemaSerialize)]
pub(crate) struct InvokeResponse {
    #[serde(rename = "Payload")]
    pub payload: &'static str,
}

fn output_buffer(bench: &mut Bencher) {
    let rpc_response = RpcResponse {
        service_method: "Function.Invoke",
        seq: 1,
        error: None,
    };
    let invoke_response = InvokeResponse {
        payload: "\"Hello world!\"",
    };

    let mut stream = StreamSerializer::new_with_buffer();
    let type_id_1 = RpcResponse::schema_register(stream.schema_mut()).unwrap();
    let type_id_2 = InvokeResponse::schema_register(stream.schema_mut()).unwrap();
    stream
        .serialize_with_type_id(type_id_1, &rpc_response)
        .unwrap();
    stream
        .serialize_with_type_id(type_id_2, &invoke_response)
        .unwrap();

    bench.iter(|| {
        {
            let output = stream.get_mut();
            let rem = output.remaining();
            output.advance(rem);
        }
        stream
            .serialize_with_type_id(type_id_1, &rpc_response)
            .unwrap();
        stream
            .serialize_with_type_id(type_id_2, &invoke_response)
            .unwrap();
    });

    assert_eq!(stream.get_ref().remaining(), 43);
}

fn output_write_vec(bench: &mut Bencher) {
    let rpc_response = RpcResponse {
        service_method: "Function.Invoke",
        seq: 1,
        error: None,
    };
    let invoke_response = InvokeResponse {
        payload: "\"Hello world!\"",
    };

    let mut stream = StreamSerializer::new_with_write(Vec::new());
    let type_id_1 = RpcResponse::schema_register(stream.schema_mut()).unwrap();
    let type_id_2 = InvokeResponse::schema_register(stream.schema_mut()).unwrap();
    stream
        .serialize_with_type_id(type_id_1, &rpc_response)
        .unwrap();
    stream
        .serialize_with_type_id(type_id_2, &invoke_response)
        .unwrap();

    bench.iter(|| {
        {
            stream.get_mut().get_mut().truncate(0);
        }
        stream
            .serialize_with_type_id(type_id_1, &rpc_response)
            .unwrap();
        stream
            .serialize_with_type_id(type_id_2, &invoke_response)
            .unwrap();
    });

    assert_eq!(stream.get_ref().get_ref().len(), 43);
}

benchmark_group!(benches, output_buffer, output_write_vec);
benchmark_main!(benches);
