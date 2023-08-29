const fs = require('fs');

const json = fs.readFileSync(`${process.cwd()}/package.json`);
const file = fs.readFileSync(`${process.cwd()}/post_run.json`);

const package = JSON.parse(json);
const template = JSON.parse(file);
const {name} = package;
const { folders, files } = template;

function runFiles(){
    for(let i=0; i<files.length; i++){
        replaceFileWithName(files[i], name);
    }
}

function runFolders(){
    for(let i=0; i<folders.length; i++){
        let ff = folders[i].replace(/ego_example/g, name);
        fs.renameSync(`${process.cwd()}/${folders[i]}`, `${process.cwd()}/${ff}`);
    }
}

function replaceFileWithName(file, p){
    p = p.replace(/-/g, '_');
    const f= fs.readFileSync(`${process.cwd()}/${file}`,{encoding:'utf-8'});
    const one = f.replaceAll(/ego_example/g, p);
    const two = one.replace(/ego-example/g, p);
    // console.log(two);
    fs.writeFileSync(`${process.cwd()}/${file}`, two);
}

function main(){
    runFiles();
    runFolders();
}

main();