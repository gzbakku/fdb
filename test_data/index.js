global.input = require('input');
global.io = require("./io");
global.faker = require('faker');
global.md5 = require('md5');
global.uniqid = require('uniqid');
global.random = (max,min)=>{
  return Math.ceil(((Math.random() * (max - min)) + min));
}

const warehouse = require("./apis/warehouse");

async function main(){

  if(true){
    return warehouse.init();
  }

  const data_type = await input.select("data type",['warehouse','files']);
  if(data_type === "warehouse"){
    warehouse.init();
  }

  main();

}

main();
