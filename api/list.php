<?php

function sort_domain($domain_list): array
{
    // Sort domain list by domain root name,then by domain name
    $domain_list = array_unique($domain_list);
    $domain_list = array_values($domain_list);
    $domain_list = array_map(function ($domain) {
        $domain = explode(".", $domain);
        return array_reverse($domain);
    }, $domain_list);
    usort($domain_list, function ($a, $b) {
        $a = implode(".", $a);
        $b = implode(".", $b);
        return strcmp($a, $b);
    });
    return array_map(function ($domain) {
        $domain = array_reverse($domain);
        return implode(".", $domain);
    }, $domain_list);
}

$domain_list = sort_domain(array(
    "contentstorage.osi.office.net",
    "onedrive.live.com",
    "login.live.com",
    "skyapi.onedrive.live.com",
    "d.docs.live.net",
    "contentsync.onenote.com",
    "hierarchyapi.onenote.com",
    "ocws.officeapps.live.com",
    "www.onenote.com",
    "config.edge.skype.com",
    "roaming.officeapps.live.com",
    "pagecontentsync.onenote.com",
    "api.onedrive.live.com",
    "c1-onenote-15.cdn.office.net",
    "docs.live.net",
    "storage.live.com",
    "skydrive.wns.windows.com",
    "client.wns.windows.com",
    "mobile.pipe.aria.microsoft.com",
    "oneclient.sfx.ms",
    "skydrivesync.policies.live.net",
    "api.onedrive.com",
    "public.bn.files.1drv.com",
    "public.dm.files.1drv.com",
    "cdn.onenote.net",
    "ad.atdmt.com",
    "officeimg.vo.msecnd.net",
    "onenote.officeapps.live.com",
    "outlook.office365.com",
    "substrate.office.com",
    "todo.microsoft.com",
    "to-do.live.com",
    "account.live.com",
    "outlook.live.com",
    "profile.live.com",
    "westeurope1-0.pushnp.svc.ms",
    "modernb.akamai.odsp.cdn.office.net",
    "ams03pap002files.storage.live.com",
    "browser.pipe.aria.microsoft.com",
));
