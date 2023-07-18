import Heading from "@aomao/plugin-heading"
import Codeblock, {CodeBlockComponent} from "@aomao/plugin-codeblock-vue";
import Math, {MathComponent} from "@aomao/plugin-math";
import {ToolbarComponent, ToolbarPlugin} from "@aomao/toolbar-vue";
import Alignment from '@aomao/plugin-alignment';
import Backcolor from '@aomao/plugin-backcolor';
import Code from "@aomao/plugin-code";
import Bold from "@aomao/plugin-bold";
import Embed, {EmbedComponent} from '@aomao/plugin-embed';
import File, {FileComponent, FileUploader} from '@aomao/plugin-file';
import Fontcolor from '@aomao/plugin-fontcolor';
import Fontsize from '@aomao/plugin-fontsize';
import Fontfamily from '@aomao/plugin-fontfamily';
import Hr, {HrComponent} from '@aomao/plugin-hr';
import Indent from '@aomao/plugin-indent';
import Italic from '@aomao/plugin-italic';
import Image, {ImageComponent, ImageUploader} from '@aomao/plugin-image';
import Link from '@aomao/plugin-link-vue';
import Lineheight from '@aomao/plugin-line-height';
import Orderedlist from '@aomao/plugin-orderedlist';
import Paintformat from '@aomao/plugin-paintformat';
import Quote from '@aomao/plugin-quote';
import Redo from '@aomao/plugin-redo';
import Removeformat from '@aomao/plugin-removeformat';
import Selectall from '@aomao/plugin-selectall';
import Strikethrough from '@aomao/plugin-strikethrough';
import Status, {StatusComponent} from '@aomao/plugin-status';
import Sub from '@aomao/plugin-sub';
import Sup from '@aomao/plugin-sup';
import Table, {TableComponent} from '@aomao/plugin-table';
import Tasklist, {CheckboxComponent} from '@aomao/plugin-tasklist';
import Underline from '@aomao/plugin-underline';
import Undo from '@aomao/plugin-undo';
import Unorderedlist from '@aomao/plugin-unorderedlist';
import Video, {VideoComponent, VideoUploader} from '@aomao/plugin-video';

class MyMath extends Math {
    query(key, code, success, failed) {
        MathJax.tex2svgPromise(code).then(node => {
            let svg = node.querySelector("svg")
            const s = new XMLSerializer().serializeToString(svg);
            const ImgBase64 = `data:image/svg+xml;base64,${window.btoa(s)}`;
            success(ImgBase64);
        })
    }
}

export const toolbarItems = [
    ["collapse"],
    ['undo', 'redo', 'paintformat', 'removeformat'],
    ['heading', 'fontfamily', 'fontsize'],
    ['bold', 'italic', 'strikethrough', 'underline', 'moremark'],
    ['fontcolor', 'backcolor'],
    ['alignment'],
    ['unorderedlist', 'orderedlist', 'tasklist', 'indent', 'line-height'],
    ['link', 'quote', 'hr']
]

export const plugins = [Link,
    MyMath, Heading, Code, Codeblock,
    Bold, ToolbarPlugin, Alignment, Backcolor,
    Embed, Fontcolor, Fontsize, Fontfamily, Hr, Indent, Italic, Lineheight,
    Orderedlist, Paintformat, Quote, Orderedlist, Paintformat, Quote, Redo, Removeformat, Selectall, Strikethrough, Status, Sub, Sup, Table, Tasklist,
    Underline, Undo, Unorderedlist, Image, ImageUploader, File, FileUploader, Video, VideoUploader]

export const cards = [
    CodeBlockComponent,
    MathComponent, ToolbarComponent, EmbedComponent, HrComponent,
    StatusComponent, TableComponent, CheckboxComponent, FileComponent, ImageComponent, VideoComponent]

export const config = {
    table: {
        cardToolbars(items) {
            let result = [];
            items.forEach(item => {
                if (item.key !== 'maximize') {
                    result.push(item)
                }
            })
            return result
        }
    },
}