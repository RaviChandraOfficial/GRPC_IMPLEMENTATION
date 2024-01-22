fn main() {
    tonic_build::compile_protos("proto/greeter.proto").unwrap(); 
                                                                // compile_protos() is used to generate rust code from protcol buffers
}
