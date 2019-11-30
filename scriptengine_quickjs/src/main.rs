#[allow(unused_imports)]
use log::{debug, info, warn, LevelFilter};

use std::ffi::{CStr, OsStr};
use std::os::raw::{c_char, c_void};
use std::os::unix::ffi::OsStrExt;
use std::path::Path;
use std::ptr::null_mut;

use failure::Error;
use structopt::StructOpt;

use qjs::{ffi, Context, ContextRef, ErrorKind, Eval, Local, Runtime, Value, WriteObj, ExtractValue};
use foreign_types::ForeignTypeRef;
use std::fs::File;
use std::io::Read;
use std::cell::RefCell;

#[derive(Debug, StructOpt)]
#[structopt(name = "qjs", about = "QuickJS script engine for OHX")]
pub struct Opt {
    /// Script arguments
    args: Vec<String>,
}

unsafe extern "C" fn jsc_module_loader(
    ctx: *mut ffi::JSContext,
    module_name: *const c_char,
    _opaque: *mut c_void,
) -> *mut ffi::JSModuleDef {
    let ctxt: &ContextRef = ForeignTypeRef::from_ptr(ctx);
    let module_name = Path::new(OsStr::from_bytes(CStr::from_ptr(module_name).to_bytes()));

    info!("load module: {:?}", module_name);

    ctxt.eval_file(module_name, Eval::MODULE | Eval::COMPILE_ONLY)
        .ok()
        .map_or_else(null_mut, |func| func.as_ptr().as_ptr())
}

fn get_url(ctx:&ContextRef, url: String) -> String {
    format!("get_url {}", url)
}

fn ruletype() -> String {
    "condition".to_owned()
}

fn set_global_var(var_name: String, var_value: String) {
    format!("setGlobalVar {} {}", var_name, var_value);
}

fn get_global_var(var_name: String) -> Value {
    format!("getGlobalVar {}", var_name);
    unimplemented!()
}

fn set_named_output(var_name: String, var_value: Value) {
    format!("setNamedOutputValue {} {:?}", var_name, var_value);
}

fn get_named_input(var_name: String) -> Value {
    format!("getNamedInputValue {}", var_name);
    unimplemented!()
}

fn exec_thing_action(thing_id: String, action_id:String,arguments:Value)  {
    format!("execThingAction {}", thing_id);
}

fn get_thing_state(thing_id: String, state_name:String,state_instance:u16) -> Value {
    format!("getThingState {}", thing_id);
    unimplemented!()
}

fn get_all_thing_states(thing_id: String)  -> Value {
    format!("getAllThingStates {}", thing_id);
    unimplemented!()
}

fn cmd_thing_state(thing_id: String, state_name: String,state_instance:u16, var_value: Value) -> bool{
    format!("cmdThingState {} {}", thing_id, state_name);
    false
}

fn register_state_listener(ctx:&ContextRef, thing_id: String, callback_function: Value) -> String{
    let r=format!("notifyOnThingStatesChange {} {}", thing_id, callback_function.is_function_bytecode());
    ctx.free_value(callback_function);
    r
}

fn unregister_state_listener(listener_id:u64) {
    format!("unregisterThingStateListener {}", listener_id);
}

fn main() -> Result<(), Error> {
    env_logger::builder().filter_level(LevelFilter::Info).init();

    let opt = Opt::from_clap(
        &Opt::clap()
            .version(qjs::LONG_VERSION.as_str())
            .get_matches(),
    );
    debug!("opts: {:?}", opt);

    let rt = Runtime::new();
    // loader for ES6 modules
    rt.set_module_loader::<()>(None, Some(jsc_module_loader), None);

    {
        let ctxt = Context::new(&rt);

        ctxt.std_add_helpers::<_, String>(None)?;
        ctxt.init_module_os()?;

        let get_url_fun: fn(&ContextRef, String) -> String = get_url;
        ctxt.global_object().set_property("get_url", get_url_fun)?;

        let register_state_listener_fun: fn(&ContextRef, String,Value) -> String = register_state_listener;
        ctxt.global_object().set_property("notifyOnThingStatesChange", register_state_listener_fun)?;

        let outside = RefCell::new(Vec::new());
        let hello_fun = |ctxt: &ContextRef, _this: Option<&Value>, args: &[Value]| {
            if let Some(abc) = String::extract_value(&ctxt.bind(&args[0])) {
                let mut vec = outside.borrow_mut();
                vec.push(abc.clone());

                let r = format!("hello {}", abc);
                r
            } else {
                String::new()
            }
        };

        let hello = ctxt.new_c_function(hello_fun, Some("hello"), 1)?;
        ctxt.global_object().set_property("hello", hello)?;

        ctxt.global_object().set_property("output", ffi::NULL)?;

        if let Some(filename) = opt.args.first() {
            debug!("eval file: {}", filename);

            let mut buf = Vec::new();
            buf.extend_from_slice("import {sleep,setTimeout,clearTimeout} from 'os';\n".as_bytes());
            File::open(filename)?.read_to_end(&mut buf)?;

            let val = ctxt.eval_script(buf, filename, Eval::MODULE | Eval::COMPILE_ONLY)?;
            let _ = ctxt.set_import_meta(&val, true, true);
            let buf = ctxt.write_object(&val, WriteObj::BYTECODE)?;
            if let Err(err) = ctxt.eval_binary(&buf, false) {
                eprintln!("error: {}", err);

                if let Some(stack) = err.downcast_ref::<ErrorKind>().and_then(|err| err.stack()) {
                    eprintln!("{}", stack)
                }
                rt.std_free_handlers();
                return Err(err);
            }
        };

        ctxt.std_loop();

        let res: Local<Value> = ctxt.global_object();
        let res = res.get_property("output");

        match res {
            Some(res) => {
                println!("b{},f{},i{},s{},{}", res.is_bool(), res.is_float(), res.is_integer(), res.is_string(), res);
            }
            None => { println!("res undefined"); }
        }
    }

    rt.std_free_handlers();

    Ok(())
}
