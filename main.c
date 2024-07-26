#include <curl/curl.h>

static CURLcode sslctx_function(CURL *curl, void *sslctx, void *parm) {
    return CURLE_OK;
}

int main(int argc, char *argv[]) {
    CURLcode ret;
    CURL *hnd;

    char *data = "";


    hnd = curl_easy_init();

    curl_easy_reset(hnd);

    curl_easy_setopt(hnd, CURLOPT_URL, "https://google.com");
    curl_easy_setopt(hnd, CURLOPT_NOPROGRESS, 1L);
    curl_easy_setopt(hnd, CURLOPT_NOBODY, 1L);
    curl_easy_setopt(hnd, CURLOPT_MAXREDIRS, 50L);
    curl_easy_setopt(hnd, CURLOPT_HTTP_VERSION, (long)CURL_HTTP_VERSION_3);
    curl_easy_setopt(hnd, CURLOPT_VERBOSE, 1L);
    curl_easy_setopt(hnd, CURLOPT_SSL_CTX_FUNCTION, *sslctx_function);
    curl_easy_setopt(hnd, CURLOPT_SSL_CTX_DATA, data);
    ret = curl_easy_perform(hnd);

    curl_easy_cleanup(hnd);

    return (int)ret;
}
