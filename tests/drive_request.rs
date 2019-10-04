use graph_error::GraphResult;
use graph_rs::prelude::*;
use std::collections::HashMap;
use std::error::Error;
use std::ffi::OsStr;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::thread;
use std::time::Duration;
use test_tools::oauthrequest::OAuthRequest;
use test_tools::support::cleanup::CleanUp;

#[test]
fn common_paths() {
    if let Some(token) = OAuthRequest::request_access_token() {
        let t = token.1.bearer_token().clone();
        get_drive(t, token.0.as_str());
        get_recent(t, token.0.as_str());
        get_root(t, token.0.as_str());
    }
}

fn get_recent(token: &str, rid: &str) {
    let client = Graph::new(token);
    if let Err(e) = client.v1().drives(rid).drive().recent().send() {
        panic!(
            "Request Error. Method: drive recent. Error: {:#?}",
            e.description()
        );
    }
}

fn get_drive(token: &str, rid: &str) {
    let client = Graph::new(token);
    if let Err(e) = client.v1().drives(rid).drive().drive().send() {
        panic!(
            "Request Error. Method: drive root. Error: {:#?}",
            e.description()
        );
    }
}

fn get_root(token: &str, rid: &str) {
    let client = Graph::new(token);
    if let Err(e) = client.v1().drives(rid).drive().root().send() {
        panic!(
            "Request Error. Method: drive root. Error: {:#?}",
            e.description()
        );
    }
}

#[test]
fn create_delete_folder() {
    OAuthRequest::test_credentials(|t| {
        if let Some((id, bearer)) = t {
            let client = Graph::new(bearer.as_str());
            let folder: HashMap<String, serde_json::Value> = HashMap::new();
            let create_folder_res = client
                .v1()
                .drives(id.as_str())
                .drive()
                .create_folder(
                    "",
                    &serde_json::json!({
                        "name": "ci_docs",
                        "folder": folder,
                        "@microsoft.graph.conflictBehavior": "rename"
                    }),
                )
                .send();

            if let Ok(response) = create_folder_res {
                let item_id = response.value().id.clone().unwrap();
                thread::sleep(Duration::from_secs(2));

                let req = client
                    .v1()
                    .drives(id.as_str())
                    .drive()
                    .delete(item_id.as_str())
                    .send();

                if let Ok(res) = req {
                    assert!(res.error().is_none());
                } else if let Err(e) = req {
                    panic!(
                        "Request error. Method: drive delete. Error: {:#?}",
                        e.description()
                    );
                }
            } else if let Err(e) = create_folder_res {
                panic!(
                    "Request error. Method: create folder. Error: {:#?}",
                    e.description()
                );
            }
        }
    });
}

#[test]
fn root_children_list_versions_get_item() {
    OAuthRequest::test_credentials(|t| {
        if let Some((id, bearer)) = t {
            let client = Graph::new(bearer.as_str());
            if let Ok(res) = client
                .v1()
                .drives(id.as_str())
                .drive()
                .root_children()
                .send()
            {
                let value = res.value().index(0).clone().unwrap();
                let item_id = value.id.clone().unwrap();

                if let Err(e) = client
                    .v1()
                    .drives(id.as_str())
                    .drive()
                    .list_versions(item_id.as_str())
                    .send()
                {
                    panic!("Request Error. Method: list versions. Error: {:#?}", e);
                }

                let req = client
                    .v1()
                    .drives(id.as_str())
                    .drive()
                    .get_item(item_id.as_str())
                    .value();

                if let Err(_) = req {
                    panic!("Request Error. Method: drive get_item");
                } else if let Ok(res) = req {
                    assert!(res.value()["name"].as_str().is_some());
                }
            } else {
                panic!("Request Error. Method: drive root children");
            }
        }
    });
}

#[test]
fn drive_check_in_out() {
    OAuthRequest::test_credentials(|t| {
        if let Some((id, bearer)) = t {
            let client = Graph::new(bearer.as_str());

            let req: GraphResult<GraphResponse<()>> = client
                .v1()
                .drives(id.as_str())
                .drive()
                .check_out(":/test_check_out_document.docx:")
                .send();

            if let Ok(res) = req {
                assert!(res.error().is_none());
            } else if let Err(e) = req {
                panic!(
                    "Request Error. Method: drive check_out. Error: {:#?}",
                    e.description()
                );
            }

            thread::sleep(Duration::from_secs(3));
            let req = client
                .v1()
                .drives(id.as_str())
                .drive()
                .check_in(
                    ":/test_check_out_document.docx:",
                    &serde_json::json!({
                        "comment": "test check in",
                    }),
                )
                .send();

            if let Ok(res) = req {
                assert!(res.error().is_none());
            } else if let Err(e) = req {
                panic!(
                    "Request Error. Method: drive check_in. Error: {:#?}",
                    e.description()
                );
            }
        }
    });
}

#[test]
fn drive_download() {
    OAuthRequest::test_credentials(|t| {
        if let Some((id, bearer)) = t {
            let file_location = "./test_files/test_document.docx";
            let mut clean_up = CleanUp::new(|| {
                let path = Path::new(file_location);
                if path.exists() {
                    std::fs::remove_file(path).unwrap();
                }
            });

            clean_up.rm_files(file_location.into());

            let client = Graph::new(bearer.as_str());
            let req: GraphResult<PathBuf> = client
                .v1()
                .drives(id.as_str())
                .drive()
                .download(":/test_document.docx:", "./test_files")
                .send();

            if let Ok(path_buf) = req {
                assert!(path_buf.exists());
            } else if let Err(e) = req {
                panic!(
                    "Request Error. Method: drive check_out. Error: {:#?}",
                    e.description()
                );
            }
        }
    });
}

#[test]
fn drive_download_format() {
    OAuthRequest::test_credentials(|t| {
        if let Some((id, bearer)) = t {
            let file_location = "./test_files/test_document.pdf";
            let mut clean_up = CleanUp::new(|| {
                let path = Path::new(file_location);
                if path.exists() {
                    std::fs::remove_file(path).unwrap();
                }
            });

            clean_up.rm_files(file_location.into());

            let client = Graph::new(bearer.as_str());
            let req: GraphResult<PathBuf> = client
                .v1()
                .drives(id.as_str())
                .drive()
                .download(":/test_document.docx:", "./test_files")
                .format("pdf")
                .rename(&OsStr::new("test_document.pdf"))
                .send();

            if let Ok(path_buf) = req {
                assert!(path_buf.exists());
                assert_eq!(path_buf.extension(), Some(OsStr::new("pdf")));
                assert_eq!(path_buf.file_name(), Some(OsStr::new("test_document.pdf")));
            } else if let Err(e) = req {
                panic!(
                    "Request Error. Method: drive check_out. Error: {:#?}",
                    e.description()
                );
            }
        }
    });
}

#[test]
fn drive_update() {
    OAuthRequest::test_credentials(|t| {
        if let Some((id, bearer)) = t {
            let client = Graph::new(bearer.as_str());
            let req = client
                .v1()
                .drives(id.as_str())
                .drive()
                .update(
                    ":/update_test_document.docx:",
                    &serde_json::json!({
                        "name": "update_test.docx"
                    }),
                )
                .value();

            if let Ok(response) = req {
                assert_eq!(response.value()["name"].as_str(), Some("update_test.docx"));
                thread::sleep(Duration::from_secs(2));

                let req = client
                    .v1()
                    .drives(id.as_str())
                    .drive()
                    .update(
                        ":/update_test.docx:",
                        &serde_json::json!({
                            "name": "update_test_document.docx"
                        }),
                    )
                    .value();

                if let Ok(response) = req {
                    assert_eq!(
                        response.value()["name"].as_str(),
                        Some("update_test_document.docx")
                    );
                } else if let Err(e) = req {
                    panic!(
                        "Request Error. Method: drive update. Error: {:#?}",
                        e.description()
                    );
                }
            } else if let Err(e) = req {
                panic!(
                    "Request Error. Method: drive check_out. Error: {:#?}",
                    e.description()
                );
            }
        }
    });
}

#[test]
fn drive_upload_new_and_replace_and_delete() {
    OAuthRequest::test_credentials(|t| {
        if let Some((id, bearer)) = t {
            let client = Graph::new(bearer.as_str());
            let upload_res = client
                .v1()
                .drives(id.as_str())
                .drive()
                .upload_new(
                    ":/test_upload_file.txt:",
                    "./test_files/test_upload_file.txt",
                )
                .unwrap()
                .value();

            if let Ok(value) = upload_res {
                assert!(value.value()["id"].as_str().is_some());
                let item_id = value.value()["id"].as_str().unwrap();

                let mut file = OpenOptions::new()
                    .write(true)
                    .open("./test_files/test_upload_file.txt")
                    .unwrap();

                file.write_all("Test Update File".as_bytes()).unwrap();
                file.sync_all().unwrap();

                let upload_replace = client
                    .v1()
                    .drives(id.as_str())
                    .drive()
                    .upload_replace(item_id, "./test_files/test_upload_file.txt")
                    .unwrap()
                    .value();

                if let Ok(value) = upload_replace {
                    let item_id2 = value.value()["id"].as_str().unwrap();
                    assert_eq!(item_id, item_id2);
                } else if let Err(e) = upload_replace {
                    panic!(
                        "Request Error. Method: drive upload replace. Error: {:#?}",
                        e.description()
                    );
                }

                thread::sleep(Duration::from_secs(2));
                let delete_res = client
                    .v1()
                    .drives(id.as_str())
                    .drive()
                    .delete(item_id)
                    .send();

                if let Ok(result) = delete_res {
                    assert!(result.error().is_none());
                } else if let Err(e) = delete_res {
                    panic!(
                        "Request Error. Method: drive delete. Error: {:#?}",
                        e.description()
                    );
                }
            } else if let Err(e) = upload_res {
                panic!(
                    "Request Error. Method: drive upload. Error: {:#?}",
                    e.description()
                );
            }
        }
    });
}
