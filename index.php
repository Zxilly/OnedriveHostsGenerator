<?php
require "dns_get.php";

function get_cache_time()
{
    $file = fopen("time.pid", "r+");
    $time_last = fread($file, filesize("time.pid"));
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

function refresh_dns_cache()
{
    global $domain_list;
    $file = fopen("dns.cache", "w+") or die("Unable to open file!");
    fwrite($file, "####### Onenote Hosts Start #######" . "\n");
    foreach ($domain_list as $domain) {
        //echo dns_get_record($domain,DNS_A)[0]['ip']." ".$domain."\n";
        //fwrite($file,"233\n");
        fwrite($file, dns_get_record($domain, DNS_A)[0]['ip'] . " " . $domain . "\n");
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
