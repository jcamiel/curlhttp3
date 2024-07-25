#include <curl/curl.h>

int main(int argc, char *argv[])
{
    CURLcode ret;
    CURL *hnd;

    hnd = curl_easy_init();

    curl_easy_reset(hnd);

    curl_easy_setopt(hnd, CURLOPT_BUFFERSIZE, 102400L);
    curl_easy_setopt(hnd, CURLOPT_URL, "https://google.com");
    curl_easy_setopt(hnd, CURLOPT_NOPROGRESS, 1L);
    curl_easy_setopt(hnd, CURLOPT_NOBODY, 1L);
    curl_easy_setopt(hnd, CURLOPT_USERAGENT, "curl/8.8.0");
    curl_easy_setopt(hnd, CURLOPT_MAXREDIRS, 50L);
    curl_easy_setopt(hnd, CURLOPT_HTTP_VERSION, (long)CURL_HTTP_VERSION_3);
    curl_easy_setopt(hnd, CURLOPT_FILETIME, 1L);
    curl_easy_setopt(hnd, CURLOPT_VERBOSE, 1L);
    curl_easy_setopt(hnd, CURLOPT_FTP_SKIP_PASV_IP, 1L);

    curl_easy_setopt(hnd, CURLOPT_COOKIEFILE, "");
    curl_easy_setopt(hnd, CURLOPT_CERTINFO, 1L);
    curl_easy_setopt(hnd, CURLOPT_SSL_VERIFYPEER, 1L);
    curl_easy_setopt(hnd, CURLOPT_SSL_VERIFYHOST, 1L);

    ret = curl_easy_perform(hnd);

    curl_easy_cleanup(hnd);

    return (int)ret;
}
