// 1Password Types

export interface Vault {
  id: string;
  name: string;
}

export interface VaultItem {
  id: string;
  title: string;
  category: string;
  vault_id: string;
  vault_name: string;
  tags: string[];
  created_at: string;
  updated_at: string;
}

export interface VaultItemField {
  id: string;
  label: string;
  value: string;
  field_type: string;
  section: string | null;
}

export interface VaultItemDetail extends VaultItem {
  fields: VaultItemField[];
}

export interface OpStatus {
  cli_installed: boolean;
  cli_version: string | null;
  signed_in: boolean;
  accounts: string[];
}
