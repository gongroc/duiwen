export default {
    namespaced: true,
    state: {
        language: "zh-CN",
        editor: {
            autoSave: false
        },
        side: {
            showFolder: true,
            showArticle: true,
            sortByDefault: true
        },
        titles: {
            showDrawerPanel: true,
            showGraph: true
        }
    },
    mutations: {
        updateEditorAutoSave(state, payload) {
            state.editor.autoSave = payload
        },
        updateLanguage(state, payload) {
            state.language = payload
        },
        toggleSideShowFolder(state) {
            state.side.showFolder = !state.side.showFolder
        },
        toggleSideShowArticle(state) {
            state.side.showArticle = !state.side.showArticle
        },
        updateSideSortType(state, payload) {
            state.side.sortByDefault = payload
        },
        toggleTitlesShowDrawerPanel(state) {
            state.titles.showDrawerPanel = !state.titles.showDrawerPanel
        },
        toggleTitlesShowGraph(state) {
            state.titles.showGraph = !state.titles.showGraph
        }
    },
}