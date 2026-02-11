// SSH Types
export interface SshHost {
  name: string;
  hostname: string;
  port: number;
  user: string;
  identity_file: string | null;
  proxy_jump: string | null;
  is_1password_agent: boolean;
  group: "direct" | "proxy" | "local" | "github";
}

export interface SshHostDetail extends SshHost {
  raw_config: string;
}
