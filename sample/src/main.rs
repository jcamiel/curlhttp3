use std::ffi::CString;
use curl::easy::Easy;
use curl_sys::CURLcode;
use libc::c_long;


fn curl_bindings(url: &str) {
    unsafe {
        curl_sys::curl_global_init(curl_sys::CURL_GLOBAL_ALL);

        let handle = curl_sys::curl_easy_init();

        let url = CString::new(url.as_str()).unwrap();

        let ret = curl_sys::curl_easy_setopt(handle, curl_sys::CURLOPT_URL, url.as_ptr());
        ok_or_exit(ret);

        let ret = curl_sys::curl_easy_setopt(handle, curl_sys::CURLOPT_NOBODY, 1 as c_long);
        ok_or_exit(ret);

        let ret = curl_sys::curl_easy_setopt(
            handle,
            curl_sys::CURLOPT_HTTP_VERSION,
            curl_sys::CURL_HTTP_VERSION_3 as c_long,
        );
        ok_or_exit(ret);

        let ret = curl_sys::curl_easy_setopt(handle, curl_sys::CURLOPT_VERBOSE, 1 as c_long);
        ok_or_exit(ret);

        let ret =
            curl_sys::curl_easy_setopt(handle, curl_sys::CURLOPT_TIMEOUT_MS, 20 * 1000 as c_long);
        ok_or_exit(ret);

        let ret = curl_sys::curl_easy_perform(handle);
        ok_or_exit(ret);
    }
}


fn curl_easy(url: &str) {
    let handle = Easy::new();

}






fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let url = &args[1];

    curl_bindings(url);
}




fn ok_or_exit(code: CURLcode) {
    if code != curl_sys::CURLE_OK {
        std::process::exit(code as i32)
    }
}
