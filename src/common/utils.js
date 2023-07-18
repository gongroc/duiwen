import {invoke} from "@tauri-apps/api";
import {ElMessage} from 'element-plus'

export const ipc = async (cmd, payload) => {
    try {
        return await invoke(cmd, payload);
    } catch (err) {
        ElMessage({
            type:"error",
            message: err,
        })
        return err
    }
}

