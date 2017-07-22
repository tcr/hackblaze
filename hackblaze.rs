fn main() {
    let exclude = vec![
        "/applications/",
        "/bin/",
        "/dev/",
        "/etc/",
        "/home/",
        "/library/",
        "/net/",
        "/opt/",
        "/private/",
        "/sbin/",
        "/system/",
        "/usr/",
    ];

    let exclude_types = "wab~,vmc,vhd,vhdx,vdi,vo1,vo2,vsv,vud,iso,dmg,sparseimage,sys,cab,exe,msi,dll,dl_,wim,ost,o,qtch,log,ithmb,vmdk,vmem,vmsd,vmsn,vmss,vmx,vmxf,menudata,appicon,appinfo,pva,pvs,pvi,pvm,fdd,hds,drk,mem,nvram,hdd";

    println!("rsync --partial --progress -avzl {exclude} {exclude_types} --stats -n / /tmp",
        exclude=exclude.iter().map(|x| format!("--exclude {:?}", x)).collect::<Vec<_>>().join(" "),
        exclude_types=exclude_types.split(",").map(|x| format!("--exclude '*.{}'", x)).collect::<Vec<_>>().join(" "),
    );
}