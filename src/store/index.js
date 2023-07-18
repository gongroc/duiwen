import {createStore} from "vuex";
import createPersistedstate from 'vuex-persistedstate'

import setting from "./setting.js";
import snapshot from "./snapshot.js";

export default createStore({
    modules: {
        setting, snapshot
    },
    plugins: [
        createPersistedstate()
    ]
})