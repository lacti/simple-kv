const net = require("net");
const mem = new Map();
const commands = {
  get: key => mem.get[key] || "NULL",
  set: (key, value) => {
    mem.set(key, value);
    return "OK";
  },
  delete: key => {
    mem.delete(key);
    return "OK";
  }
};

net
  .createServer()
  .on("connection", client => {
    let buffer = "";
    client.on("data", chunk => {
      buffer += chunk.toString("utf8");
      const lines = buffer.split(/\r?\n/);
      buffer = lines.pop();
      lines.forEach(command => {
        const args = command.split(/\s+/);
        const result = commands[args.shift().toLowerCase()](...args);
        client.write(result + "\r\n", "utf8", error =>
          error ? console.error(error) : undefined
        );
      });
    });
  })
  .on("error", console.error)
  .listen(6378, () => console.log("Server is ready on 6378"));
