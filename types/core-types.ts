export type Status = 'success' | 'failed' | 'loading' | 'idle' 

export interface State {
	status: Status,
    message?: String,
    data?: Object
}

export interface TriggerKeyMenuAnimationIndex  {
    copy: boolean,
    remove: boolean,
}

// New interface for the data structure returned by the backend
export interface SSHKeyInfo {
    filename: string,
    key_info: string,
}

export interface Key {
    keyPid: string,
    publicKey: string,
    email: string,
    isActive: boolean,
    keyType: string,
    filename: string,
    assignedHosts: HostEntry[],
}

export interface HostEntry {
    host: string,
    hostname: string | null,
    user: string | null,
    identity_file: string | null,
    identities_only: boolean,
}