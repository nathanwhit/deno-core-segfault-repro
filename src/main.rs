use deno_core::{op2, v8, JsRuntime, RuntimeOptions};
fn main() {
    #[op2(fast)]
    #[smi]
    fn op_nop(a: u32, b: u32) -> usize {
        a as usize + b as usize
    }
    deno_core::extension!(
        test_ext,
        ops = [op_nop],
        esm_entry_point = "ext:test_ext/setup",
        esm = ["ext:test_ext/setup" = {
            source = r#"
                globalThis.fastFn = Deno.core.ops.op_nop;
            "#
        }]
    );

    let mut runtime = JsRuntime::new(RuntimeOptions {
        extensions: vec![test_ext::init_ops_and_esm()],
        module_loader: None,
        ..Default::default()
    });
    let scope = &mut runtime.handle_scope();

    let code =
        r#"let count = 0; for(let i=0; i < 1000; i++) { const l = fastFn(i, 2 * i); count += l; }"#;

    let code = v8::String::new(scope, code).unwrap();
    let script = v8::Script::compile(scope, code, None).unwrap();
    for _ in 0..100 {
        script.run(scope);
    }
}
