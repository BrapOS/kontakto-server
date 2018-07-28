extern crate futures;
extern crate grpcio;
extern crate protobuf;

use std::io::Read;
use std::sync::Arc;
use std::{io, thread};

use futures::Future;
use futures::sync::oneshot;
use grpcio::{Environment, RpcContext, ServerBuilder, UnarySink};

#[path="../kontakto-common/protos/kontakto.rs"]
mod kontakto;
#[path="../kontakto-common/protos/kontakto_grpc.rs"]
mod kontakto_grpc;

use kontakto::{CreateContactRequest, CreateContactReply,
               UpdateContactRequest, UpdateContactReply,
               DeleteContactRequest, DeleteContactReply,
               GetAllContactsRequest, GetAllContactsReply};

use kontakto_grpc::Kontakto;

#[derive(Clone)]
struct KontaktoService;

impl Kontakto for KontaktoService {
    fn create_contact(&self, ctx: RpcContext, req: CreateContactRequest, sink: UnarySink<CreateContactReply>) {
        println!("Received a `create_contact` request");
        let resp = CreateContactReply::new();
        let f = sink.success(resp)
            .map_err(move |e| println!("failed to reply {:?}: {:?}", req, e));
        ctx.spawn(f)
    }
    fn update_contact(&self, ctx: RpcContext, req: UpdateContactRequest, sink: UnarySink<UpdateContactReply>) {
        println!("Received a `update_contact` request");
        let resp = UpdateContactReply::new();
        let f = sink.success(resp)
            .map_err(move |e| println!("failed to reply {:?}: {:?}", req, e));
        ctx.spawn(f)
    }
    fn delete_contact(&self, ctx: RpcContext, req: DeleteContactRequest, sink: UnarySink<DeleteContactReply>) {
        println!("Received a `delete_contact` request");
        let resp = DeleteContactReply::new();
        let f = sink.success(resp)
            .map_err(move |e| println!("failed to reply {:?}: {:?}", req, e));
        ctx.spawn(f)
    }
    fn get_all_contacts(&self, ctx: RpcContext, req: GetAllContactsRequest, sink: UnarySink<GetAllContactsReply>) {
        println!("Received a `get_all_contacts` request");
        let resp = GetAllContactsReply::new();
        let f = sink.success(resp)
            .map_err(move |e| println!("failed to reply {:?}: {:?}", req, e));
        ctx.spawn(f)
    }
}

fn main() {
    let env = Arc::new(Environment::new(1));
    let service = kontakto_grpc::create_kontakto(KontaktoService);
    let mut server = ServerBuilder::new(env)
        .register_service(service)
        .bind("127.0.0.1", 50051)
        .build()
        .unwrap();
    server.start();
    for &(ref host, port) in server.bind_addrs() {
        println!("listening on {}:{}", host, port);
    }
    let (tx, rx) = oneshot::channel();
    thread::spawn(move || {
        println!("Press ENTER to exit...");
        let _ = io::stdin().read(&mut [0]).unwrap();
        tx.send(())
    });
    let _ = rx.wait();
    let _ = server.shutdown().wait();
}
