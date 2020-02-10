const md5 = require('md5');
const faker = require('faker');
const number_of_users_per_collection = 5;
const io = require('./io');

let collection_paths = [
  '/cities/delhi',
  '/country/ghaziabad',
  '/country/gurgoan',
  '/country/noida',
];

make_collections();

async function make_collections(){
  let collections = {};
  for(let path of collection_paths){
    let collection = {
      id:md5(path),
      path:path,
      users:make_users()
    };
    collections[collection.id] = collection;
  }

  const cwd = io.cwd;
  const path = cwd + "/sample_store_data.json";

  const write = await io.write(path,collections);
  if(!write){
    console.log('!!! failed to write sample data to disk path : ' + path);
  } else {
    console.log('>>> sample store data generated');
  }

}

function make_users(){
  let users = [];
  for(let i=0;i<number_of_users_per_collection;i++){
    users.push(make_user());
  }
  return users;
}

function make_user(){
  let user = {
    name:faker.name.findName(),
    email:faker.internet.email(),
    company:faker.company.companyName()
  };
  return user;
}
