use ic_cdk;
use dkim::email::Email;
use dkim::dkim::PublicKey;



#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}
const public_key_json:&str = r#"
{"Status":0,"TC":false,"RD":true,"RA":true,"AD":false,"CD":false,"Question":[{"name":"20230601._domainkey.gmail.com.","type":16}],"Answer":[{"name":"20230601._domainkey.gmail.com.","type":16,"TTL":3600,"data":"v=DKIM1; k=rsa; p=MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAntvSKT1hkqhKe0xcaZ0x+QbouDsJuBfby/S82jxsoC/SodmfmVs2D1KAH3mi1AqdMdU12h2VfETeOJkgGYq5ljd996AJ7ud2SyOLQmlhaNHH7Lx+Mdab8/zDN1SdxPARDgcM7AsRECHwQ15R20FaKUABGu4NTbR2fDKnYwiq5jQyBkLWP+LgGOgfUF4T4HZb2PY2bQtEP6QeqOtcW4rrsH24L7XhD+HSZb1hsitrE0VPbhJzxDwI4JF815XMnSVjZgYUXP8CxI1Y0FONlqtQYgsorZ9apoW1KPQe8brSSlRsi9sXB/tu56LmG7tEDNmrZ5XUwQYUUADBOu7t1niwXwIDAQAB"}],"Comment":"Response from 216.239.38.10."}"#;

#[ic_cdk::query]
fn parseEmail() -> String {
    
    let mail = "Received: by mail-oi1-f177.google.com with SMTP id e4so8660662oib.1\r\n        for <mubelotix@mubelotix.dev>; Tue, 30 Jun 2020 01:43:28 -0700 (PDT)\r\nX-Google-DKIM-Signature: v=1; a=rsa-sha256; c=relaxed/relaxed;\r\n        d=1e100.net; s=20161025;\r\n        h=x-gm-message-state:mime-version:from:date:message-id:subject:to;\r\n        bh=5NNwu8gdOD3ZoZD58FM4gy7PeYn+BudAJmLL+5Moe58=;\r\n        b=SEIL6qJEGH/+sVou4i84kC4vEEsLShYrmKLAlM/7V1fIIbpyQWDRpehMKnlGFKmTCx\r\n         Mz1NijW6tbjDJ+1eF3aE/MNSzhim2eO4JmcK5kZ4vlZzzPWE+GacZqc3QNtAufgA/EqP\r\n         eWTuFSPtSY2vHJdRX21vq8WpP31KdG0JKcv3ZykDqH0y1dAM1sAGR3Gmrcyu+HGA9Ug5\r\n         BrYx1ZPyjYOtlXEiGqaKRsrBlB5P42n2aU0TwZYrEVi9N5TULM4bS+bLtP3FmxP7uIP2\r\n         ZKuFKbcTTveG3+DaaOE7HK/dHXWXZZC9RaS/yzGettgXiwmaAENcONpTwg1jD70DU5a9\r\n         DYHg==\r\nX-Gm-Message-State: AOAM533sOvLV7q5oj9SIWatwQ3kCiOgSZHBhJb0R93ImzSZav4QObpV2\r\n        pLSheyz34dtdedvMg8G3go4HsIP3ytqkN8f9j+ZTvFkx\r\nX-Google-Smtp-Source: ABdhPJzLJRsIQigY2u6fwn04UxksGTqbklM5igDK5fVI2kljDUPeTOPWxkM4IEUQpRb6Ciacz58Kj9Dqy61/LiiyDyA=\r\nX-Received: by 2002:aca:d681:: with SMTP id n123mr15403808oig.82.1593506599851;\r\n Tue, 30 Jun 2020 01:43:19 -0700 (PDT)\r\nMIME-Version: 1.0\r\nFrom: Mubelotix <mubelotix@gmail.com>\r\nDate: Tue, 30 Jun 2020 10:43:08 +0200\r\nMessage-ID: <CANc=2UXAvRBx-A7SP9JWm=pby29s_zdFvfMDUprZ+PN_8XuO+w@mail.gmail.com>\r\nSubject: Test email\r\nTo: mubelotix@mubelotix.dev\r\nContent-Type: multipart/alternative; boundary=\"000000000000d4d95805a9492a3c\"\r\n\r\n--000000000000d4d95805a9492a3c\r\nContent-Type: text/plain; charset=\"UTF-8\"\r\n\r\nTest body\r\n\r\n--000000000000d4d95805a9492a3c\r\nContent-Type: text/html; charset=\"UTF-8\"\r\n\r\n<div dir=\"ltr\">Test body</div>\r\n\r\n--000000000000d4d95805a9492a3c--";
    let mut mail = Email::try_from(mail).unwrap();
    let key = base64::decode("MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAgxoGICfYbPE2Z75oNqCxt559UcIOBuh6RmvIrAWSIgGfFHGFiksNS/uRNOM+JAHh7UbHZtCdT5nYpNuIFboOH8TxGVw58D3dFoi97llInbHpuxcQMmVErHiEZ/5rWtCKjBE851EFU4G/1YwR+PsO7/lB5+VnU3yb0s4YcbalsY+5IKIO/ocVXBaWqu471hGAPs4GyziuZ/I40xd5N2qi5Ws9uWOnJ/NFeuKCK+l7jOY0catqheft95CIVPR0d5ihuM1bRjS/mOKhDlj/ru8emmaCzeqToUshl8LT4HZ3YVhFiM1NEj7OYDcQibIFd61ENNHc21+TOwLq3pvSZN96vwIDAQAB").unwrap();
    let public_key = PublicKey::try_from(
        public_key_json
    ).unwrap();
    return match mail.verify_with_public_key(
        &public_key
    ) {
        Ok(_) => "ok".to_string(),
        Err(_) => "error".to_string()
        
    }

   
}




// need this to generate candid

ic_cdk::export_candid!();