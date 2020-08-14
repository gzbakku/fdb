const request = require("request-promise");
const faker = require('faker');
const md5 = require('md5');
const uid = require('uniqid');
const start = now();

let location = 'files';
let no_of_objects = 5;
let log_the_list = false;

function control(){
  if(true){main();}
  if(false){read();}
  if(false){simple();}
  if(false){list();}
  if(false){kill();}
  if(false){check(1000);}
}

let base = 'http://';
function url(a){
  return base + a;
}

const write_url = url('127.0.0.1:8088/write');
const read_url = url('127.0.0.1:8088/read');
const kill_url = url('127.0.0.1:8088/kill');
const check_url = url('127.0.0.1:8088/check');
const list_url = url('127.0.0.1:8088/list');

let collection = '281a82d9c457f64718cc04053876c1e6';
if(true){
  location = 'vault';
  collection = '52a461fbb9e84de28a61645dc007048c';
}

let actor,composer,node,session;
if(true){
  actor = {
    id:'aXjD1ulK7VDP7yZRtmjVkbL6tMCUIhi5',
    sig:'XW5L4OBPjuRcLhNUAvi40mOG3RdeJ6Pb'
  };
  composer = {
    ip:'127.0.0.1',
    id:'aXjD1ulK7VDP7yZRtmjVkbL6tMCUIhi5',
    sig:'XW5L4OBPjuRcLhNUAvi40mOG3RdeJ6Pb'
  };
  node = {
    id:'aXjD1ulK7VDP7yZRtmjVkbL6tMCUIhi5',
    sig:'XW5L4OBPjuRcLhNUAvi40mOG3RdeJ6Pb'
  };
  session = {
    id:'XW5L4OBPjuRcLhNUAvi40mOG3RdeJ6Pb',
    sig:'aXjD1ulK7VDP7yZRtmjVkbL6tMCUIhi5'
  };
}

control();

function now(){
  return new Date().getTime();
}

if(false){
  setInterval(()=>{
    main();
  }, 1000);
}

async function list(){

  let start = now();

  if(!location){
    location = 'files';
  }

  console.log(">>> fetching list = " + location);

  const result = await send({
    url:list_url,
    body:{
      location:location
    },
    type:'master'
  },false);

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

  if(false){
    no_of_objects = 1;
  }

  let objects = [],ids = [];
  for(let i=0;i<no_of_objects;i++){
    let data = {
      name:faker.name.findName(),
      email:faker.internet.email(),
      uid:md5(faker.random.uuid())
    };
    ids.push(data.uid);
    if(location == 'vault'){
      let file = {
        file:data.uid,
        nonce:md5(uid()),
        hex:Buffer.from(JSON.stringify(data), 'utf8').toString('hex')
      };
      //console.log(JSON.parse(Buffer.from(file.hex,'hex').toString('utf-8')));
      objects.push(file);
    } else {
      objects.push(data);
    }
  }

  let combine = true;
  let result;

  if(combine){
    let collect = [];
    let index = 0;
    for(let file of objects){
      collect.push({
        name:file.file,
        collection:collection,
        data:file
      });
    }
    result = await write({files:collect})
    .then((r)=>{
      return true;
    })
    .catch((e)=>{
      console.log(e);
      return false;
    });
  }

  //return true;

  //console.log(objects);

  if(!combine){
    let promises = [];
    for(let file of objects){
      promises.push(write({files:[{
        name:file.file,
        collection:collection,
        data:file
      }]}));
    }
    result = await Promise.all(promises)
    .then((r)=>{
      return true;
    })
    .catch((e)=>{
      console.log(e);
      return false;
    });
  }

  const end = new Date().getTime();
  const diff = start - end;

  console.log("");
  console.log("");
  console.log("write time : " + diff);
  console.log("");
  console.log("");

  if(result && true){
    read_these(ids,combine);
  }

}

async function read_these(ids,combine,log){

  let read_start = new Date().getTime();

  console.log("");
  console.log('________________________________________________');
  console.log("_________________read_these_____________________");
  console.log('________________________________________________');
  console.log("");
  console.log("");

  console.log('reading : ' + ids.length);

  let promises = [];

  if(combine){
    promises.push(send({
      url:read_url,
      body:{
        files:ids,
        collection:collection
      },
      type:'master'
    },true,true));
  } else {
    for(let id of ids){
      promises.push(read(id));
    }
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

  //log = true;

  if(!object){
    object = {
      name:'akku',
      uid:'sad89as7d89a7dasqada',
      message:"Lorem Ipsum is simply dummy text of the printing and typesetting industry. Lorem Ipsum has been the industry's standard dummy text ever since the 1500s, when an unknown printer took a galley of type and scrambled it to make a type specimen book. It has survived not only five centuries, but also the leap into electronic typesetting, remaining essentially unchanged. It was popularised in the 1960s with the release of Letraset sheets containing Lorem Ipsum passages, and more recently with desktop publishing software like Aldus PageMaker including versions of Lorem Ipsum.c"
    };
  }

  //console.log(write_url);

  //let body = object.file + '---' + JSON.stringify(object)

  const result = await send({
    url:write_url,
    body:object,
    type:'master'
  });

  if(log){
    console.log(result);
  }

  return true;

}

async function read(id,log){

  if(!log){
    log = false;
  }

  if(!id){
    id = '0c78aac7c09ad9625409610ad259242b';
  }

  const result = await send({
    url:read_url,
    body:{
      files:[id],
      collection:collection
    },
    type:'master'
  },log,log);

  //console.log(file);

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

async function check(no,log,track,error){
  if(!no){
    no = 1;
  }
  if(!log){
    log = false;
  }
  if(!track){
    track = false;
  }
  if(!error){
    error = false;
  }
  let start = now();
  let promises = [];
  for(let i=0;i<no;i++){
    promises.push(send({
      url:check_url,
      body:{},
      type:'master'
    },log,track,error));
  }
  let mid = now();
  let mid_time = (mid - start) / 1000;
  console.log("request sent in : " + mid_time + ' secs');
  let result = await Promise.all(promises)
  .then((r)=>{
    let success = 0,failed = 0;
    for(let i of r){
      if(i.result == 'success'){
        success++;
      } else {
        failed++;
      }
    }
    return {success:success,failed:failed};
  })
  .catch((e)=>{
    console.log(e);
    console.log('failed to send all check request');
    return false;
  });
  if(!result){
    return false;
  } else {
    console.log(result);
  }
  let end = now();
  let end_time = (end - start) / 1000;
  console.log("check complete in : " + end_time + ' secs');
}

async function kill(){
  send({
    url:kill_url,
    body:{},
    type:'node'
  },true,true);
}

async function send(data,log,track,error){

  return new Promise((resolve,reject)=>{

    let time = now();
    let ruid = uid();
    let base_sig = md5(actor.sig + session.sig);
    let request_sig = md5(time + ruid + base_sig);
    let body_sig = md5(JSON.stringify(data.body) + base_sig);
    let node_sig = md5(time + ruid + node.sig);
    let master_sig = md5(time + ruid + composer.sig);

    let json = true;
    if(data.write){
      json = false;
    }

    let build = {
      rejectUnauthorized:false,
      uri:data.url,
      body:data.body,
      json:true
    };

    if(false){
      console.log(build);
    }

    if(data.hasOwnProperty('type')){
      if(data.type == 'master'){
        build.headers = {
          fdb_app_type:'master',
          timestamp:time,
          ruid:ruid,
          composer_id:composer.id,
          req_signature:master_sig
        };
      }
      if(data.type == 'node'){
        build.headers = {
          fdb_app_type:'node',
          timestamp:time,
          ruid:ruid,
          req_signature:node_sig,
          node_id:node.id
        };
      }
    }

    //console.log(build);

    request.post(build)
    .then((r)=>{
      if(log){
        console.log(r);
      }
      if(track){
        let final = (now() - time) / 1000;
        console.log("completed in  : " + final + " secs");
      }
      resolve(r);
    })
    .catch((e)=>{
      if(error){
        console.log(e);
      }
      reject(e)
    });


    if(false){
      if(result){
        return result;
      } else {
        return {};
      }
      return result;
    }

  });

}
