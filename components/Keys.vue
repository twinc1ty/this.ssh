<template>
  <div class="container">
    <div v-if="keysState.status === 'idle'" class="idle-state">
      <p class="text-center">{{ keysState.message }}</p>
      <p class="text-center">
        <button @click="fetchSSHKeys">Fetch SSH Keys</button>
      </p>
    </div>

    <div v-if="keysState.status === 'loading'" class="loading-state">
      <p class="text-center">{{ keysState.message }}</p>
    </div>

    <div v-if="keysState.status === 'failed'" class="error-state">
      <p class="text-center">Error: {{ keysState.data }}</p>
    </div>

    <div v-if="keysState.status === 'success'">
      <ul class="keys-list">
        <li v-for="(key, index) in keys" :key="index" class="key-item">
          <key-card
            :key-details="key"
            :is-active="isThisKeyActive(key)"
            @key-removed="handleKeyRemoved"
          />
        </li>
      </ul>

      <div
        v-if="props.isDebugMode"
        class="debug-section mt-4 p-4 bg-gray-800 rounded"
      >
        <h3 class="text-white mb-2">Debug Info:</h3>
        <p class="text-gray-300 text-sm">Total keys: {{ keys.length }}</p>
        <div
          v-for="(key, index) in keys"
          :key="index"
          class="text-gray-400 text-xs mt-2"
        >
          <p>Key {{ index + 1 }}:</p>
          <p>keyPid: {{ key.keyPid }}</p>
          <p>email: {{ key.email }}</p>
          <p>keyType: {{ key.keyType }}</p>
          <p>publicKey: {{ key.publicKey.substring(0, 50) }}...</p>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import type { Key, State, SSHKeyInfo } from "~/types/core-types";

const keys = ref<Key[]>([]);
const loadedKeys = ref<Key[]>([]);
const error = ref<string | null>(null);
const commandList = <string[]>["get_ssh_keys", "get_loaded_ssh_agent_keys"];

interface Props {
  isDebugMode: boolean;
}

const props = defineProps<Props>();

const keysState = ref<State>({
  status: "idle",
  data: {},
  message: "Idling, waiting for input",
});

const loadedKeysState = ref<State>({
  status: "idle",
  data: {},
  message: "Idling, waiting for input",
});

onMounted(() => {
  fetchSSHKeys();
  getLoadedSSHAgentKeys();
});

const isThisKeyActive = (key: Key): boolean => {
  return loadedKeys.value.some((loadedKey) => loadedKey.keyPid === key.keyPid);
};

const formatKeys = (keys: SSHKeyInfo[]): any[] => {
  const isActive = false;
  return keys.map((key) => {
    if (
      key.key_info.startsWith("Error for") ||
      key.key_info.startsWith("Failed to run")
    ) {
      return {
        keyPid: "Error",
        publicKey: key.key_info,
        email: "error@example.com",
        isActive,
        keyType: "error",
        filename: key.filename,
      };
    }

    // The ssh-keygen -lv output format is:
    // "256 SHA256:TIPy/ZP3JraYYuXYbnFNanxouYhl1Wmov4aJOWcw4x8 bro@test.com (ED25519)"
    // Format: [key_size] [hash] [email] (key_type)

    // Split by spaces
    const parts = key.key_info.split(" ");

    if (parts.length < 4) {
      return {
        keyPid: key.key_info.substring(0, 20) + "...",
        publicKey: key.key_info,
        email: "unknown@example.com",
        isActive,
        keyType: "unknown",
        filename: key.filename,
      };
    }

    // The first part is the key size
    const keyPid = parts[0] || "";

    // The email is the third part (index 2)
    // Format: [0: key_size], [1: hash], [2: email], [3: (key_type)]
    const email = parts[2] || "unknown@example.com";

    // Extract key type from the last part (e.g., "(ED25519)" -> "ED25519")
    const lastPart = parts[parts.length - 1] || "";
    const keyTypeMatch = lastPart.match(/\(([^)]+)\)/);
    const keyType = keyTypeMatch ? keyTypeMatch[1] : "unknown";

    // The public key hash is the second part
    const publicKey = parts[1] || key.key_info.substring(0, 50) + "...";

    const result = {
      keyPid,
      publicKey,
      email,
      isActive,
      keyType,
      filename: key.filename,
    };
    return result;
  });
};

// Separate function for formatting loaded SSH agent keys
const formatLoadedKeys = (keys: string[]): any[] => {
  return keys.map((key) => {
    // The ssh-add -l output format is different:
    // "256 SHA256:TIPy/ZP3JraYYuXYbnFNanxouYhl1Wmov4aJOWcw4x8 /path/to/key (ED25519)"
    // Format: [key_size] [hash] [path] (key_type)

    // Split by spaces
    const parts = key.split(" ");

    if (parts.length < 4) {
      return {
        keyPid: key.substring(0, 20) + "...",
        publicKey: key,
        email: "unknown@example.com",
        isActive: true,
        keyType: "unknown",
        filename: "unknown", // We don't have filename for loaded keys
      };
    }

    // The first part is the key size
    const keyPid = parts[0] || "";

    // The path is the third part (index 2)
    const path = parts[2] || "";

    // Extract filename from path
    const filename = path.split("/").pop() || "unknown";

    // Extract key type from the last part (e.g., "(ED25519)" -> "ED25519")
    const lastPart = parts[parts.length - 1] || "";
    const keyTypeMatch = lastPart.match(/\(([^)]+)\)/);
    const keyType = keyTypeMatch ? keyTypeMatch[1] : "unknown";

    // The public key hash is the second part
    const publicKey = parts[1] || key.substring(0, 50) + "...";

    const result = {
      keyPid,
      publicKey,
      email: "unknown@example.com",
      isActive: true,
      keyType,
      filename,
    };
    return result;
  });
};

const fetchSSHKeys = async () => {
  useStateModifier(keysState.value, "loading", "Loading...", {});
  error.value = null;
  try {
    const result = await invoke<SSHKeyInfo[]>(commandList[0]);

    if (!result || !Array.isArray(result)) {
      console.error("Invalid result format:", result);
      useStateModifier(
        keysState.value,
        "failed",
        "Invalid data format received",
        {
          error: "Invalid data format",
        }
      );
      return;
    }

    keys.value = formatKeys(result);

    if (keys.value.length === 0) {
      useStateModifier(keysState.value, "failed", "No SSH keys found", {});
    } else {
      useStateModifier(
        keysState.value,
        "success",
        "SSH keys loaded successfully",
        {
          keys: keys.value,
        }
      );
    }
  } catch (err: any) {
    console.error("Error fetching SSH keys:", err);
    useStateModifier(keysState.value, "failed", "Could not fetch SSH Keys", {
      error: err,
    });
    error.value = err.message || "Failed to fetch SSH keys";
  }
};

const getLoadedSSHAgentKeys = async () => {
  useStateModifier(
    loadedKeysState.value,
    "loading",
    "Loading SSH Agent Keys...",
    {}
  );
  error.value = null;
  try {
    const result = await invoke<string[]>(commandList[1]);

    loadedKeys.value = formatLoadedKeys(result);

    if (loadedKeys.value.length === 0) {
      useStateModifier(
        loadedKeysState.value,
        "failed",
        "No SSH Agent keys found",
        {
          keys: loadedKeys.value,
        }
      );
    } else {
      useStateModifier(
        loadedKeysState.value,
        "success",
        "SSH Agent keys loaded successfully",
        {
          keys: loadedKeys.value,
        }
      );
    }
  } catch (err: any) {
    useStateModifier(
      loadedKeysState.value,
      "failed",
      "Could not fetch SSH Agent Keys",
      {
        error: err,
      }
    );
    error.value = err.message || "Failed to fetch SSH agent keys";
  }
};

const handleKeyRemoved = () => {
  // Refresh the keys list after a key is removed
  fetchSSHKeys();
  getLoadedSSHAgentKeys();
};

// Expose methods to parent component
defineExpose({
  fetchSSHKeys,
  getLoadedSSHAgentKeys,
});
</script>
