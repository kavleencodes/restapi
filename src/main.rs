use postgre::{Client,NoTls};
use postgre::Error as PostgresError;
use std::net::{Tcplisterner,Tcpstream};
use std::io::{Read,Write};
use std::env;

#[macro_use]
extern crate serde_derive;

#[derive(Serialize,Deserialize)]
struct user {
    id: Option<i32>,
    name:String,
    email:String,  

}

const db_url:&str=env!("database_url");

const ok_response:&str=


const not_found:&str=
"not found error 404";

cost internal_error:&str="internal error";

fn main(){
     if let err(_)=set_databasr(){
        println!("error setting database");
        return;
     }

     let listerner=Tcplisterner::bind(format!("0.0.0.0:8080")).unwrap;
     println!("server listening to port 8080");

     for stream in listerner.incoming(){
        match stream{
            Ok(stream)=>{
                handle_client(stream);
            }
            err(e) =>{
                println!("enable to connect {}",e);
            }
        }
     }
 }

     fn set_database() {}


fn get_id(request: &str) ->&str{
    request.split("/").nth(4).unwrap_or_default().split_whitespace().next().unwrap_or_default()
}

fn get_user_request_body(request:&str)->Result<User,serde_json::Error>{
    serde_json::from_str(request.split("/r/n/r/n").last().unwrap_or_default())
}

fn handle_client(mut stream:TcpStream){
    let mut buffer=[0,1024];
    let mut request=String::new();

    match stream.read(&mut buffer){
        Ok (size)=>{
            request.push_str(String::from_utf8_lossy(&buffer[..size]).as_ref());

            let(status_line, content)=match &*request{
                r if r.starts_with("OPTIONS")=>(ok_response.to_string(),"".to_string()),
                r if r.starts_with("POST/api/rust/users")=> handle_post_request(r),
                r if r.starts_with("GET/api/rust/users/")=> handle_get_request(r),
                r if r.starts_with("GET/api/rust/users")=> handle_get_all_request(r),
                r if r.starts_with("PUT/api/rust/users")=> handle_put_request(r),
                r if r.starts_with("DELETE/api/rust/users")=> handle_delete_request(r),
                _=>(not_found.to_string(),"404 not found".to_string()),
            };
            stream.write_all(format!("{}{}",status_line,content).as_bytes()).unwrap();
            }
            err(e)=> eprintln!("unable to read stream:{}",e),
        }

    }

// to retrieve user data 
fn handle_post_request(request:&str) ->(String,String){
    match (get_user_request_body(request),Client::connect(db_url,NoTls)){
        (Ok(user), Ok(mut client))=>{
            let row=Client.query_one(
                "insert into users (name,email) values($1,$2) returning id",
                &[&user.name,&user.email]
            )
            .unwrap();


            let user_id=row.get(0);

            match client.query_one("SELECT id,name,email FROM users WHERE"){
                Ok(row) =>{
                    let user=User{
                        id:Some(row.get(0)),
                        name: row.get(1),
                        email: row.get(2),

                    };
                    (ok_response.to_string(), serde_json::to_string(&user).unwrap())
                }
                err(_)=>
                (internal_error.to_string,"failed to retrieve created user".to_string()),
            }
        }
        _=> (internal_error.to_string(),"internal error".to_string()),
    }

}


fn handle_get_request(request:&str)->(String,String){
    match (get_id(&request).parse::<i32>,Client::connect(db_url)) {
        (Ok(id), Ok(mut client))=>
          match client.query_one("SELECT *FROM users WHERE id=$1",&[&id]){
            Ok(row)=> let user=User{
                id:Some(row.get(0)),
                name: row.get(1),
                email: row.get(2),

            };
            (ok_response.to_string(), serde_json::to_string(&user).unwrap())
        }
        err(_)=>
        (internal_error.to_string,"user not found".to_string()),
    }

_=> (internal_error.to_string(),"internal error".to_string()),
}


//to get all the request
fn handle_get_request(request:&str)->(String,String){
    match Client::connect(db_url,NoTls){
        Ok(mut client)=>{
            let mut users=Vec::new();
            for row in client.query("SELECT id,name,email FROM users ",&[]).unwrap(){
                users.push(User{
                    id:row.get(0),
                    name: row.get(1),
                    email: row.get(2),
                });
            }
        
         
            (ok_response.to_string(), serde_json::to_string(&user).unwrap())
        }

_=> (internal_error.to_string(),"internal error".to_string()),
 }
}


handle_put_request(request:&str)->(String,String){
    match(
        get_id(&request).parse::<i32>(),
        get_user_request_body(&request),
        Client::connect(db_url,NoTls),
    )
    {
        (Ok(id), Ok(user), Ok(mut client))=>{
            client.execute(
                "UPDATE users SET name=$1,email=$2 Where id=$3",
                &[&user.name,$user.email,&id]
            )
            .unwrap();

        
            (ok_response.to_string(), serde_json::to_string(&user).unwrap())
        }

_=> (internal_error.to_string(),"internal error".to_string()),
 }
}

fn handle_delete_request(request: &str) -> (String, String) {
    match (get_id(&request).parse::<i32>(), Client::connect(DB_URL, NoTls)) {
        (Ok(id), Ok(mut client)) => {
            let rows_affected = client.execute("DELETE FROM users WHERE id = $1", &[&id]).unwrap();

            if rows_affected == 0 {
                return (not_found.to_string(), "User not found".to_string());
            }

            (ok_response.to_string(), "User deleted".to_string())
        }
        _ => (internal_error.to_string(), "Internal error".to_string()),
    }
}


          
         



