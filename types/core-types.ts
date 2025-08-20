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
    filename: string, // Add filename for key removal
}