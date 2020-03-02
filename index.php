<?php
require "dns_get.php";
//include "dns_get.php";


function get_cache_time()
{
    $file = fopen("time.pid","r+");
    $time_last = fread($file,filesize("time.pid"));
    $time_now = mktime();
    fclose($file);
    return $time_now-$time_last;
}

function refresh_time()
{
    $file = fopen("time.pid","w+");
    fwrite($file,mktime());
    fclose($file);
}

function refresh_dns_cache()
{
    global $domain_list;
    $file = fopen("dns.cache","w+") or die("Unable to open file!");
    fwrite($file,"####### Onenote Hosts Start #######"."\n");
    foreach ($domain_list as $domain)
    {
        //echo dns_get_record($domain,DNS_A)[0]['ip']." ".$domain."\n";
        //fwrite($file,"233\n");
        fwrite($file,dns_get_record($domain,DNS_A)[0]['ip']." ".$domain."\n");
    }
    fwrite($file,"####### Onenote Hosts End #######"."\n");
    fclose($file);
    refresh_time();
}

function read_dns_cache()
{
    $file = fopen("dns.cache","r+");
    while(!feof($file))
    {
        echo fgets($file)."<br>";
    }
    fclose($file);
}

if(get_cache_time()>3600)
{
    refresh_dns_cache();
    read_dns_cache();
}
else
{
    read_dns_cache();
}






/*
echo "####### Onenote Hosts Start #######"."<br>";

foreach ($domain_list as $domain)
{
    echo dns_get_record($domain,DNS_A)[0]['ip']." ".$domain."<br>";
}

echo "####### Onenote Hosts End #######"."<br>";
*/
//print_r($host1);
/*
echo "onedrive.live.com ".$host1[0]['ip'];
echo "<br>";
//print_r($host2);
echo "skyapi.onedrive.live.com ".$host2[0]['ip'];
echo "<br>";
echo "api.onedrive.live.com ".$host3[0]['ip'];
*/
/*
echo <<<DNS
{$host1[0]['ip']} onedrive.live.com <br>
{$host2[0]['ip']} skyapi.onedrive.live.com <br>
{$host3[0]['ip']} api.onedrive.live.com <br>
{$host4[0]['ip']} d.docs.live.net <br>
{$host5[0]['ip']} contentsync.onenote.com <br>
{$host6[0]['ip']} hierarchyapi.onenote.com <br>
{$host7[0]['ip']} ocws.officeapps.live.com <br>
{$host8[0]['ip']} www.onenote.com <br>
{$host9[0]['ip']} config.edge.skype.com <br>
{$host10[0]['ip']} roaming.officeapps.live.com <br>
{$host11[0]['ip']} pagecontentsync.onenote.com <br>
DNS;*/

