import { Pool } from "pg";

let conn;

// Create the connection if it's not created already
if (!conn) {
  const sslOption = process.env.POSTGRES_SSL === "true"
    ? { rejectUnauthorized: false }
    : false;

  conn = new Pool({
    user: process.env.POSTGRES_USER,
    password: process.env.POSTGRES_PASSWORD,
    host: process.env.POSTGRES_HOST,
    port: Number(process.env.POSTGRES_PORT),
    database: process.env.POSTGRES_DATABASE,
    ssl: sslOption,
  });
}

export default conn as Pool;
