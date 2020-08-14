const io = require('./io');
const uniqid = require('uniqid');
const md5 = require('md5');

const no_of_objects = 500;

main();

async function main(){

  let collect = [];
  for(let i=0;i<no_of_objects;i++){
    collect.push(md5(uniqid()));
  }

  const path = io.cwd + '/sample/list.json'
  const run = await io.write(path,collect);
  if(!run){
    console.log('failed write to path :  ' + path);
    return false;
  } else {
    console.log('file written');
  }

}
