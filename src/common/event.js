export class Event {
    events = {}
    static instance = null;
    constructor() {
        if (Event.instance !== null) {
            return Event.instance
        }
        return Event.instance = this
    }
    subscribe(event, callback) {
        if (this.events[event]) {
            this.events[event].push(callback);
        } else {
            this.events[event] = [callback]
        }
    }
    publish(event, ...args) {
        const subscribedEvents = this.events[event];

        if (subscribedEvents && subscribedEvents.length) {
            subscribedEvents.forEach(callback => {
                callback.call(this, ...args);
            });
        }
    }
    unsubscribe(event, callback) {
        const subscribedEvents = this.events[event];
        if (subscribedEvents && subscribedEvents.length) {
            this.events[event] = this.events[event].filter(cb => cb !== callback)
        }
    }
}

export const Topic = {
    LANGUAGE_CHANGED: "LANGUAGE_CHANGED",
    FOLDER_CREATED: "FOLDER_CREATED",
    FOLDER_DELETED: "FOLDER_DELETED",
    FOLDER_UNLINKED: "FOLDER_UNLINKED",
    ARTICLE_CREATED: "ARTICLE_CREATED",
    ARTICLE_DELETED: "ARTICLE_DELETED",
    SIDE_CFG_CHANGED: "SIDE_CFG_CHANGED",
    ARTICLE_TITLE_CHANGED: "ARTICLE_TITLE_CHANGED",
    ARTICLE_CONTENT_SAVED: "ARTICLE_CONTENT_SAVED",
    ARTICLE_CONTENT_CHANGED: "ARTICLE_CONTENT_CHANGED",
    ARTICLE_INTRO_CHANGED: "ARTICLE_INTRO_CHANGED",
    COMMAND_OPEN_FOLDER: "COMMAND_OPEN_FOLDER",
    COMMAND_OPEN_ARTICLE: "COMMAND_OPEN_ARTICLE",
    COMMOND_SAVE_ARTICLE: "COMMOND_SAVE_ARTICLE",
}