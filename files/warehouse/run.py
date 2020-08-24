import subprocess;
from threading import Thread

# cargo watch -x "run -- -i=529f07b0b1d5d3df52b0440bad708090 -k=192.168.0.1 -p=5701 -t=2dc6bb40e73417bba878d3c8e3e08780 -c=192.168.0.1:5200 -n=5100 -d='d://workstation/expo/rust/fdb/data'"

base_port = 5701;
no_of_actors = 1;

def make_cmd(port):
    base_cmd = 'cargo watch -x "run -- -i=529f07b0b1d5d3df52b0440bad708090 -k=192.168.0.1 -p={} -t=2dc6bb40e73417bba878d3c8e3e08780 -c=192.168.0.1:5200 -n=5100 -d=d://workstation/expo/rust/fdb/data"'
    return base_cmd.format(port);

def run_cmd(cmd):
    print(subprocess.Popen(cmd, shell=True, stdout=subprocess.PIPE).stdout.read())

threads = [];
for i in range(0,no_of_actors):
    cmd = make_cmd(base_port);
    hold = Thread(target=run_cmd,args=(cmd,));
    hold.start();
    threads.append(hold);
    base_port += 1;

for thread in threads:
    thread.join();
