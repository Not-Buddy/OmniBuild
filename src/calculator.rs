// src/calculator.rs
use std::sync::{Mutex, OnceLock};
use jni::objects::{JValue, JString, JObject};
use jni::{JNIEnv, JavaVM, InitArgsBuilder, JNIVersion};
use jni::sys::jint;

static JVM: OnceLock<JavaVM> = OnceLock::new();
static JVM_ENV_MUTEX: Mutex<()> = Mutex::new(());

fn with_attached_env<F, R>(f: F) -> Result<R, Box<dyn std::error::Error>>
where
    F: FnOnce(&mut JNIEnv) -> Result<R, Box<dyn std::error::Error>>,
{
    let classpath = "-Djava.class.path=src/JAVA_Code";
    
    let jvm = JVM.get_or_init(|| {
        let jvm_args = InitArgsBuilder::new()
            .version(JNIVersion::V8)
            .option(classpath)
            .build()
            .unwrap();
        
        JavaVM::new(jvm_args).unwrap()
    });

    let _guard = JVM_ENV_MUTEX.lock().unwrap();
    let mut env = jvm.attach_current_thread()?;
    f(&mut env)
}

/// Call: public static long Fibonacci.fibonacci(int n)
pub fn java_fibonacci(n: i32) -> i64 {
    with_attached_env(|env| {
        let class = env.find_class("Fibonacci")?;  // No "JAVACode/" prefix
        let result = env.call_static_method(class, "fibonacci", "(I)J", &[JValue::Int(n)])?;
        Ok(result.j()?)
    }).unwrap_or(0)
}

/// Call: public static boolean PrimeChecker.isPrime(long n)
pub fn java_is_prime(n: i64) -> bool {
    with_attached_env(|env| {
        let class = env.find_class("PrimeChecker")?;  // No "JAVACode/" prefix
        let result = env.call_static_method(class, "isPrime", "(J)Z", &[JValue::Long(n)])?;
        Ok(result.z()?)
    }).unwrap_or(false)
}

/// Call: public static double ArrayStats.median(double[] arr)
pub fn java_array_median(arr: &[f64]) -> f64 {
    with_attached_env(|env| {
        let class = env.find_class("ArrayStats")?;  // No "JAVACode/" prefix
        let arr_obj = env.new_double_array(arr.len() as jint)?;
        env.set_double_array_region(&arr_obj, 0, arr)?;
        
        let arr_as_obj = JObject::from(arr_obj);
        let result = env.call_static_method(class, "median", "([D)D", &[JValue::Object(&arr_as_obj)])?;
        Ok(result.d()?)
    }).unwrap_or(0.0)
}

/// Call: public static String StringOps.uppercase(String s)
pub fn java_string_uppercase(s: &str) -> String {
    with_attached_env(|env| {
        let class = env.find_class("StringOps")?;  // No "JAVACode/" prefix
        let input = env.new_string(s)?;
        let input_obj = JObject::from(input);
        
        let result = env.call_static_method(
            class, 
            "uppercase", 
            "(Ljava/lang/String;)Ljava/lang/String;", 
            &[JValue::Object(&input_obj)]
        )?;
        
        let output: JString = result.l()?.into();
        let output_string: String = env.get_string(&output)?.into();
        Ok(output_string)
    }).unwrap_or_else(|_| s.to_string())
}

// [Rest of your C/C++/Assembly functions remain unchanged...]


// ------- Your C/C++/ASM bindings (unchanged) ---------

unsafe extern "C" {
    fn c_add(a: f64, b: f64) -> f64;
    fn c_subtract(a: f64, b: f64) -> f64;
    fn c_multiply(a: f64, b: f64) -> f64;
    fn c_divide(a: f64, b: f64) -> f64;
}

pub fn add(a: f64, b: f64) -> f64 { unsafe { c_add(a, b) } }
pub fn subtract(a: f64, b: f64) -> f64 { unsafe { c_subtract(a, b) } }
pub fn multiply(a: f64, b: f64) -> f64 { unsafe { c_multiply(a, b) } }
pub fn divide(a: f64, b: f64) -> f64 { unsafe { c_divide(a, b) } }

unsafe extern "C" {
    fn cpp_power(base: f64, exponent: f64) -> f64;
    fn cpp_sqrt(x: f64) -> f64;
}

pub fn power(base: f64, exponent: f64) -> f64 { unsafe { cpp_power(base, exponent) } }
pub fn sqrt(x: f64) -> f64 { unsafe { cpp_sqrt(x) } }

unsafe extern "C" {
    fn asm_fast_and(a: u64, b: u64) -> u64;
    fn asm_fast_or(a: u64, b: u64) -> u64;
    fn asm_fast_xor(a: u64, b: u64) -> u64;
}

pub fn bitwise_and(a: u64, b: u64) -> u64 { unsafe { asm_fast_and(a, b) } }
pub fn bitwise_or(a: u64, b: u64) -> u64 { unsafe { asm_fast_or(a, b) } }
pub fn bitwise_xor(a: u64, b: u64) -> u64 { unsafe { asm_fast_xor(a, b) } }
