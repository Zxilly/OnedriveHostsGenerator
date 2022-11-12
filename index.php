<?php
require "list.php";

function get_cache_time()
{
    // try open, if not exist, create
    $file = fopen("time.pid", "a+");
    if (filesize("time.pid") == 0) {
        fwrite($file, 0);
        $time_last = 0;
    } else {
        $time_last = fread($file, filesize("time.pid"));
    }

    $time_now = time();
    fclose($file);
    return $time_now - $time_last;
}

function refresh_time()
{
    $file = fopen("time.pid", "w+");
    fwrite($file, time());
    fclose($file);
}

function sort_domain($domain_list): array
{
    // Sort domain list by domain root name,then by domain name
    $domain_list = array_unique($domain_list);
    $domain_list = array_values($domain_list);
    $domain_list = array_map(function ($domain) {
        $domain = explode(".", $domain);
        $domain = array_reverse($domain);
        return $domain;
    }, $domain_list);
    usort($domain_list, function ($a, $b) {
        $a = implode(".", $a);
        $b = implode(".", $b);
        return strcmp($a, $b);
    });
    $domain_list = array_map(function ($domain) {
        $domain = array_reverse($domain);
        return implode(".", $domain);
    }, $domain_list);
    return $domain_list;
}

function refresh_dns_cache()
{
    global $domain_list;
    $domain_list = sort_domain($domain_list);

    $file = fopen("dns.cache", "w+") or die("Unable to open file!");
    fwrite($file, "####### Onenote Hosts Start #######" . "\n");
    foreach ($domain_list as $domain) {
        $ip = dns_get_record($domain, DNS_A)[0]['ip'];
        $ip = str_pad($ip, 15);
        fwrite($file,  $ip. " " . $domain . "\n");
    }
    fwrite($file, "####### Onenote Hosts End #######" . "\n");
    fclose($file);
    refresh_time();
}

function read_dns_cache()
{
    $file = fopen("dns.cache", "r+");
    while (!feof($file)) {
        echo fgets($file) . "<br>";
    }
    fclose($file);
}

if (get_cache_time() > 60000) {
    refresh_dns_cache();
    read_dns_cache();
} else {
    read_dns_cache();
}
