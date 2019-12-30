const request = require("request-promise");
const faker = require('faker');
const md5 = require('md5');
const uid = require('uniqid');

let write_url = 'http://127.0.0.1:8088/write/json';
let read_url = 'http://127.0.0.1:8088/read/json';
let kill_url = 'http://127.0.0.1:8088/kill';
let check_url = 'http://127.0.0.1:8088/check';

let location = 'files';

let actor = {
  id:'aXjD1ulK7VDP7yZRtmjVkbL6tMCUIhi5',
  sig:'XW5L4OBPjuRcLhNUAvi40mOG3RdeJ6Pb'
};

let session = {
  id:'XW5L4OBPjuRcLhNUAvi40mOG3RdeJ6Pb',
  sig:'aXjD1ulK7VDP7yZRtmjVkbL6tMCUIhi5'
};

/*
cargo run -- --secure=Om2lPq84vgIhsPEhWsh3LdRmNmI2MXpQ --signature=XW5L4OBPjuRcLhNUAvi40mOG3RdeJ6Pb --id=aXjD1ulK7VDP7yZRtmjVkbL6tMCUIhi5" +
" --session_id=XW5L4OBPjuRcLhNUAvi40mOG3RdeJ6Pb --session_signature=aXjD1ulK7VDP7yZRtmjVkbL6tMCUIhi5 --base_dir=d://workstation/expo/rust/fdb/composer/instance --port=8081 --composer=127.0.0.1
*/

if(false){
  write_url = 'http://127.0.0.1:8088/write/encrypted';
  read_url = 'http://127.0.0.1:8088/read/encrypted';
  location = 'vault';
}

const start = new Date().getTime();

let no_of_objects = 5;
let log_the_list = false;

//main();
//simple();
//list();
//kill();
check();

if(false){
  setInterval(()=>{
    main();
  }, 1000);
}

function now(){
  return new Date().getTime();
}

function simple(){
  write();
  read();
}

async function list(){

  let start = now();

  if(!location){
    location = 'files';
  }

  console.log(">>> fetching list = " + location);

  const result = await request.post({
    uri:'http://127.0.0.1:8088/list',
    body:{
      location:location
    },
    json:true
  });

  let fetched = now();

  console.log(">>> fetch complete for " + result.data.length + " files in : " + ((fetched - start) / 1000) + " Sec");

  let no_of_files = 1000;
  //no_of_files = result.data.length;

  if(result.data.length < no_of_files){
    no_of_files = result.data.length;
  }

  console.log(">>> reading : " + no_of_files + ' files');

  let promises = [];
  for(let i=0;i<no_of_files;i++){
    promises.push(read(result.data[i],log_the_list));
  }

  let success = 0;
  let failed = 0;

  let read_result = await Promise.all(promises)
  .then((r)=>{
    for(let i of r){
      if(i){
        success++;
      } else {
        failed++;
      }
    }
    return true;
  })
  .catch((e)=>{
    //console.log(e);
    return false;
  });

  let read_complete = now();

  console.log(">>> read complete in : " + ((read_complete - fetched) / 1000) + " Sec");

  console.log({
    read_result:read_result,
    success:success,
    failed:failed
  });

  return true;

}

async function main(){

  console.log("");
  console.log('________________________________________________');
  console.log("________________writing_these___________________");
  console.log('________________________________________________');
  console.log("");
  console.log("");

  console.log("");
  console.log("");
  console.log("writing : " + no_of_objects);
  console.log("");
  console.log("");

  let objects = [];
  for(let i=0;i<no_of_objects;i++){
    objects.push({
      name:faker.name.findName(),
      email:faker.internet.email(),
      uid:md5(faker.random.uuid())
    });
  }

  let promises = [],ids = [];
  for(let m of objects){
    ids.push(m.uid);
    promises.push(write(m));
  }

  let result = await Promise.all(promises)
  .then((r)=>{
    return true;
  })
  .catch((e)=>{
    console.log(e);
    return false;
  });

  const end = new Date().getTime();
  const diff = start - end;

  console.log("");
  console.log("");
  console.log("write time : " + diff);
  console.log("");
  console.log("");

  if(result){
    read_these(ids);
  }

}

async function read_these(ids){

  let read_start = new Date().getTime();

  console.log("");
  console.log('________________________________________________');
  console.log("_________________read_these_____________________");
  console.log('________________________________________________');
  console.log("");
  console.log("");

  console.log('reading : ' + ids.length);

  let promises = [];

  for(let id of ids){
    promises.push(read(id));
  }

  const read_diff = read_start - new Date().getTime();

  console.log("");
  console.log("");
  console.log("read time : " + read_diff);
  console.log("");
  console.log("");

  console.log("");
  console.log("");
  console.log('________________________________________________');
  console.log("_________________read_result____________________");
  console.log('________________________________________________');
  console.log("");
  console.log("");

  let result = await Promise.all(promises)
  .then((r)=>{
    return r;
  })
  .catch((e)=>{
    console.log(e);
    return false;
  });

  let success = 0;
  let failed = 0;

  const diff = start - new Date().getTime();

  for(let r of result){
    if(r){
      success++;
    } else {
      failed++;
    }
  }

  console.log("");
  console.log("");

  console.log({
    diff:diff,
    success:success,
    failed:failed
  });

}

async function write(object,log){

  //console.log(">>> writing = " + object.uid);

  if(!object){
    object = {
      name:'akku',
      uid:'sad89as7d89a7dasqada',
      message:"Lorem Ipsum is simply dummy text of the printing and typesetting industry. Lorem Ipsum has been the industry's standard dummy text ever since the 1500s, when an unknown printer took a galley of type and scrambled it to make a type specimen book. It has survived not only five centuries, but also the leap into electronic typesetting, remaining essentially unchanged. It was popularised in the 1960s with the release of Letraset sheets containing Lorem Ipsum passages, and more recently with desktop publishing software like Aldus PageMaker including versions of Lorem Ipsum.c"
    };
  }

  //console.log(write_url);

  const result = await send({
    url:write_url,
    body:{
      file:object.uid,
      data:object
    }
  });

  if(log){
    console.log(
      ">>> http://127.0.0.1:8088/write = " +
      object.uid +
      " " +
      JSON.stringify(result)
    );
  }

  return true;

}

async function read(id,log){

  if(!log){
    log = false;
  }

  if(!id){
    id = 'sad89as7d89a7dasqada';
  }

  //console.log(">>> http://127.0.0.1:8088/read");

  const result = await send({
    url:read_url,
    body:{
      file:id
    }
  });

  if(log){
    console.log(
      ">>> " + read_url + " = " +
      id +
      " " +
      JSON.stringify(result)
    );
  }

  if(result.result == 'success'){
    return true;
  } else {
    return false;
  }

}

async function check(){
  send({
    url:check_url,
    body:{},
    type:'master'
  },true);
}

async function kill(){
  send({
    url:kill_url,
    body:{},
    type:'master'
  },true);
}

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
