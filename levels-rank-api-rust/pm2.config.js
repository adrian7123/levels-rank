const argEnvIndex = process.argv.indexOf("--env");
let argEnv = (argEnvIndex !== -1 && process.argv[argEnvIndex + 1]) || "";

const RUN_ENV_MAP = {
  local: {
    instances: 1,
    max_memory_restart: "250M",
  },
  dev: {
    instances: 1,
    max_memory_restart: "250M",
  },
  prod: {
    instances: 1,
    max_memory_restart: "1000M",
  },
};

if (!(argEnv in RUN_ENV_MAP)) {
  argEnv = "prod";
}

module.exports = {
  apps: [
    {
      name: "levels-rank-api-rust",
      script:
        "chmod 777 ./target/release/levels-rank-api-rust && ./target/release/levels-rank-api-rust",
      args: ["", ""],
      instances: RUN_ENV_MAP[argEnv].instances,
      exec_mode: "fork", //'cluster',
      watch: false,
      max_memory_restart: RUN_ENV_MAP[argEnv].max_memory_restart,
      env_local: {
        APP_ENV: "local",
      },
      env_dev: {
        APP_ENV: "dev",
      },
      env_prod: {
        APP_ENV: "prod",
      },
    },
  ],
};
