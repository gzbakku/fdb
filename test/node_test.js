const request = require("request-promise");
const faker = require('faker');
const md5 = require('md5');
const uid = require('uniqid');

let base_url = 'http://127.0.0.1:8088/';



async function send(data,log){

  let time = now();
  let ruid = uid();
  let base_sig = md5(actor.sig + session.sig);
  let request_sig = md5(time + ruid + base_sig);
  let body_sig = md5(JSON.stringify(data.body) + base_sig);

  let build = {
    uri:data.url,
    body:data.body,
    json:true
  };

  if(data.hasOwnProperty('type')){
    if(data.type == 'master'){
      build.headers = {
        fdb_app_type:'master',
        session:session.id,
        timestamp:time,
        ruid:ruid,
        req_signature:request_sig,
        body_signature:body_sig
      };
    }
  }

  //console.log(build);

  const result = await request.post(build);
  if(log){
    console.log(result);
  }

  if(result){
    return result;
  } else {
    return {};
  }

}
