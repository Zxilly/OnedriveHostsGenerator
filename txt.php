<?php
$host1 = dns_get_record("onedrive.live.com",DNS_A);
$host2 = dns_get_record("skyapi.onedrive.live.com",DNS_A);
$host3 = dns_get_record("api.onedrive.live.com",DNS_A);
$host4 = dns_get_record("d.docs.live.net",DNS_A);
$host5 = dns_get_record("contentsync.onenote.com",DNS_A);
$host6 = dns_get_record("hierarchyapi.onenote.com",DNS_A);
$host7 = dns_get_record("ocws.officeapps.live.com",DNS_A);
$host8 = dns_get_record("www.onenote.com",DNS_A);
$host9 = dns_get_record("config.edge.skype.com",DNS_A);
$host10 = dns_get_record("roaming.officeapps.live.com",DNS_A);
//print_r($host1);
/*
echo "onedrive.live.com ".$host1[0]['ip'];
echo "<br>";
//print_r($host2);
echo "skyapi.onedrive.live.com ".$host2[0]['ip'];
echo "<br>";
echo "api.onedrive.live.com ".$host3[0]['ip'];
*/
echo <<<DNS
{$host1[0]['ip']} onedrive.live.com
{$host2[0]['ip']} skyapi.onedrive.live.com
{$host3[0]['ip']} api.onedrive.live.com
{$host4[0]['ip']} d.docs.live.net
{$host5[0]['ip']} contentsync.onenote.com
{$host6[0]['ip']} hierarchyapi.onenote.com
{$host7[0]['ip']} ocws.officeapps.live.com
{$host8[0]['ip']} www.onenote.com
{$host9[0]['ip']} config.edge.skype.com
{$host10[0]['ip']} roaming.officeapps.live.com
DNS;