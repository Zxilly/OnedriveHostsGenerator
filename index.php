<?php
require "list.php";

function render(): string
{
    global $domain_list;

    $ret = "####### Onenote Hosts Start #######" . "\n";
    $ret .= "# This file is generated by https://github.com/Zxilly/OnedriveHostsGenerator" . "\n";
    $ret .= "# Generate time: " . date("Y-m-d H:i:s") . "\n";

    foreach ($domain_list as $domain) {
        $dns = dns_get_record($domain, DNS_A);

        if (!$dns) {
            $ret .= "# " . $domain . " is not resolved" . "\n";
            continue;
        }

        $ips = array_map(function ($ip) {
            return $ip['ip'];
        }, $dns);

        $ips = array_unique($ips);

        foreach ($ips as $ip) {
            $pad_ip = str_pad($ip, 15);
            $ret .= $pad_ip . " " . $domain . "\n";
        }
    }

    $ret .= "####### Onenote Hosts End #######" . "\n";

    return $ret;
}
function handler(): void
{
    header("Content-Type: text/plain");
    header("Cache-Control: s-maxage=5000, stale-while-revalidate=1000");

    echo render();
}

handler();
