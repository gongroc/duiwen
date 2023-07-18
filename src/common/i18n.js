import {createI18n} from 'vue-i18n'
import lang from "./lang.json"

const i18n = createI18n({
    locale: 'zh-CN',
    fallbackLocale: "zh-CN",
    messages: lang
})

export default i18n