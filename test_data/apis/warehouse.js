const num_of_files = 10;
const no_of_items_per_file = 10;
const file_types = ['list','book','dict','config','map','index'];
const actions = ['add_item','','',''];

module.exports = {

  init:async ()=>{

    const data_type = await input.select('data type',['value(hash)','item(index)']);
    if(data_type === "value(hash)"){
      make_values();
    }
    if(data_type === "item(index)"){
      make_items();
    }

  }

};

//this is hash
async function make_values(){
  const path = io.cwd + '/data/warehouse.json'
  const items = make_files("list",true);
  await io.write(path,items);
}

async function make_items(){
  const path = io.cwd + '/data/warehouse.json'
  const items = make_files("list",false);
  await io.write(path,items);
}

function make_files(file_type,values){
  let items = [];
  for(var i=0;i<num_of_files;i++){
    for(let item of make_file(file_type,values)){
      items.push(item);
    }
  }
  return items;
}

function make_file(file_type,values){
  const file_name = md5(uniqid()) + '-0_' + (no_of_items_per_file + 100);
  let items = [];
  for(var i=0;i<no_of_items_per_file;i++){
    let build = {
      file_name:file_name,
      file_type:file_type,
      item_index:i.toString(),
      item_value:make_data()
    };
    let item = {
      type:'add_item'
    };
    if(values){
      build["item_index"] = md5(uniqid());
    }
    item["data"] = build;
    items.push(item);
  }
  return items;
}

function make_data(){
  return JSON.stringify({
    name:faker.name.findName(),
    address:faker.address.secondaryAddress(),
    company:faker.company.companyName(),
    jobTitle:faker.name.jobTitle(),
  });
}
