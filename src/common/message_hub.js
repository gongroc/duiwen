import {listen} from "@tauri-apps/api/event"
import {ipc} from "./utils.js";

let handler = async (payload, event) => {
    switch (event) {
        case "AlertMessage": {
            break
        }
        case "StatusBarMessage": {
            break
        }
        case "JumpPage": {
            break
        }
    }
}

export default async function () {
    try {
        await listen("message_hub", async event => {
            console.log("收到消息:", event)
            let content = event.payload
            await handler(content.payload, content.event)
        })
        await ipc("message_hub")
    } catch (err) {
        console.log("绑定hub失败:", err)
    }
}