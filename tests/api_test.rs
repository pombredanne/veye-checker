extern crate hyper;
extern crate veye_checker;

use std::error::Error;
use veye_checker::{api, configs};


#[test]
fn test_api_encoding_product_key(){
    assert_eq!(api::encode_prod_key("dot.net"), "dot~net");
    assert_eq!(api::encode_prod_key("slash/net"), "slash:net");
    assert_eq!(api::encode_prod_key("dot.net/slash"), "dot~net:slash");
}

#[test]
#[cfg(feature="api")]
fn test_api_call_fetch_product_by_sha(){

    let file_sha = "5675fd96b29656504b86029551973d60fb41339b";
    let confs = configs::read_configs();

    let res = api::fetch_product_by_sha(&confs.api, file_sha).expect("Failed fetch SHA");

    let prod_url = "https://www.versioneye.com/Java/commons-beanutils/commons-beanutils".to_string();
    assert_eq!(Some(prod_url), res.url);
    assert_eq!(true, res.sha.is_some());

    let sha = res.sha.unwrap();
    assert_eq!("jar".to_string(), sha.packaging);
    assert_eq!("sha1".to_string(), sha.method);
    assert_eq!(file_sha.to_string(), sha.value);
    assert_eq!(None, sha.filepath);

    assert_eq!(true, res.product.is_some());
    let prod = res.product.unwrap();
    assert_eq!("Java".to_string(), prod.language);
    assert_eq!("Maven2".to_string(), prod.prod_type.unwrap());
    assert_eq!("commons-beanutils/commons-beanutils".to_string(), prod.prod_key);
    assert_eq!("1.7.0".to_string(), prod.version);
    assert_eq!("".to_string(), prod.name);
}


#[test]
#[cfg(feature="api")]
fn test_api_call_fetch_product(){
    let confs = configs::read_configs();
    let res = api::fetch_product(
        &confs.api, "Java", "commons-beanutils/commons-beanutils", "1.7.0"
    ).expect("Failed to fetch product details");

    assert_eq!(false, res.sha.is_some());
    assert_eq!(true, res.product.is_some());

    let prod = res.product.unwrap();
    assert_eq!("java".to_string(), prod.language);
    assert_eq!("Maven2".to_string(), prod.prod_type.unwrap());
    assert_eq!("commons-beanutils/commons-beanutils".to_string(), prod.prod_key);
    assert_eq!("1.7.0".to_string(), prod.version);
    assert_eq!("commons-beanutils".to_string(), prod.name);
}

#[test]
#[cfg(feature="api")]
fn test_api_call_fetch_product_details_by_sha(){
    let file_sha = "5675fd96b29656504b86029551973d60fb41339b";
    let confs = configs::read_configs();

    let res = api::fetch_product_by_sha(&confs.api, file_sha).expect("Failed fetch SHA");

    let prod_url = "https://www.versioneye.com/Java/commons-beanutils/commons-beanutils".to_string();
    assert_eq!(Some(prod_url), res.url);
    assert_eq!(true, res.sha.is_some());

    let sha = res.sha.unwrap();
    assert_eq!("jar".to_string(), sha.packaging);
    assert_eq!("sha1".to_string(), sha.method);
    assert_eq!(file_sha.to_string(), sha.value);
    assert_eq!(None, sha.filepath);

    assert_eq!(true, res.product.is_some());
    let prod = res.product.unwrap();
    assert_eq!("Java".to_string(), prod.language);
    assert_eq!("Maven2".to_string(), prod.prod_type.unwrap());
    assert_eq!("commons-beanutils/commons-beanutils".to_string(), prod.prod_key);
    assert_eq!("1.7.0".to_string(), prod.version);
    assert_eq!("".to_string(), prod.name);
}


#[test]
fn test_api_process_sha_response(){
    let file_sha = "5675fd96b29656504b86029551973d60fb41339b";
    let res_body = r#"
    [{
        "language":"Java",
        "prod_key":"commons-beanutils/commons-beanutils",
        "version":"1.7.0",
        "group_id":"commons-beanutils",
        "artifact_id":"commons-beanutils",
        "classifier":null,"packaging":"jar",
        "prod_type":"Maven2",
        "sha_value":"5675fd96b29656504b86029551973d60fb41339b",
        "sha_method":"sha1"
    }]
    "#;

    let res = api::process_sha_response(Some(res_body.to_string()));
    if let Some(prod_match) = res.ok() {

        assert_eq!(true, prod_match.sha.is_some());
        let sha = prod_match.sha.unwrap();
        assert_eq!("jar".to_string(), sha.packaging);
        assert_eq!("sha1".to_string(), sha.method);
        assert_eq!(file_sha.to_string(), sha.value);
        assert_eq!(None, sha.filepath);

        assert_eq!(true, prod_match.product.is_some());
        let prod = prod_match.product.unwrap();
        assert_eq!("Java".to_string(), prod.language);
        assert_eq!("Maven2".to_string(), prod.prod_type.unwrap());
        assert_eq!("commons-beanutils/commons-beanutils".to_string(), prod.prod_key);
        assert_eq!("1.7.0".to_string(), prod.version);
        assert_eq!("".to_string(), prod.name);
    }
}

#[test]
fn test_api_process_sha_response_with_empty_result(){
    let res = api::process_sha_response(Some("".to_string()));
    assert_eq!(true, res.is_err());
    let e = res.err().unwrap();
    println!("message: {}", e.description());
}

#[test]
fn test_api_process_sha_response_with_api_error(){
    let body_txt = r#"
        {"error": "Failed to match it"}
    "#;
    let res = api::process_sha_response(Some(body_txt.to_string()));
    assert_eq!(true, res.is_err());
}
