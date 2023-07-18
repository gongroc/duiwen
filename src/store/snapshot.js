export default {
    namespaced: true,
    state: {
        article: [], independents: []
    },
    mutations: {
        pushArticle(state, {id, content}) {
            let flag = true
            for (let article of state.article) {
                if (article.id === id) {
                    article.content = content
                    flag = false
                    break
                }
            }
            if (flag) {
                state.article.push({
                    id, content
                })
            }
        },
        removeArticle(state, id) {
            let index = state.article.findIndex(item => item.id === id)
            if (index) {
                state.article.splice(index, 1)
            }
        },
        removeAllArticle(state) {
            state.article = []
        },
        pushIndependent(state, id) {
            state.independents.push(id)
        },
        removeIndependent(state, id) {
            let index = state.independents.findIndex(i => i === id)
            if (index) {
                state.independents.splice(index, 1)
            }
        },
        removeAllIndependent(state) {
            state.independents = []
        }
    },

}