<template>
  <div class="editor" v-if="editor">
    <menu-bar class="editor_header" :editor.sync="editor"/>
    <editor-content class="editor_content" :editor="editor"/>
    <div class="editor_footer">
      <div class="editor_count">
        字数统计：{{ editor.storage.characterCount.characters() }}
      </div>
    </div>
  </div>
</template>

<script setup>
import {reactive, ref} from "vue";
import {Editor,EditorContent} from "@tiptap/vue-3";
import StarterKit from '@tiptap/starter-kit'
import TaskList from '@tiptap/extension-task-list'
import TaskItem from '@tiptap/extension-task-item'
import Highlight from '@tiptap/extension-highlight'
import CharacterCount from '@tiptap/extension-character-count'
import MenuBar from './menu-bar.vue'

let editor = reactive(new Editor({
  editorProps: {
    editable: function () {
      return true;
    }
  },
  extensions: [
    StarterKit.configure({
      history: false,
    }),
    Highlight,
    TaskList,
    TaskItem,
    CharacterCount.configure({
      limit: 20000,
    }),
  ],
}));
</script>

<script>
export default {
  name: 'WEditor',

  mounted() {
    window.addEventListener('keydown', this.onKeyDown, true);
  },

  methods: {
    onKeyDown(event) {
      if ((event.ctrlKey || event.metaKey) && event.keyCode === 83) {
        event.preventDefault();
      }
    }
  },
}
</script>

<style lang="scss">
.editor {
  display: flex;
  flex-direction: column;
  max-height: 60rem;
  color: #0D0D0D;
  background-color: #FFF;
  border: 1px solid #0D0D0D;
  border-radius: 0.75rem;
  text-align: left;
  min-height: 40rem;
  min-width: 400px;
  max-width: 40%;
  margin-left: 30%;
  margin-right: 30%;

  &_header {
    display: flex;
    align-items: center;
    flex: 0 0 auto;
    flex-wrap: wrap;
    padding: 0.25rem;
    border-bottom: 1px solid #0D0D0D;
  }

  &_content {
    padding: 1rem;
    flex: 1 1 auto;
    overflow-x: hidden;
    overflow-y: auto;
    -webkit-overflow-scrolling: touch;
  }

  &_footer {
    display: flex;
    flex: 0 0 auto;
    align-items: center;
    justify-content: space-between;
    flex-wrap: wrap;
    white-space: nowrap;
    border-top: 1px solid #0D0D0D;
    font-size: 12px;
    font-weight: 600;
    color: #0D0D0D;
    padding: 0.25rem 0.75rem;
  }
}
</style>

<style lang="scss">
/* Basic editor styles */
.ProseMirror:focus {
  outline: none;
}

.ProseMirror {
  min-height: 560px;

  > * + * {
    //margin-top: 0.75em;
  }

  p {
    margin-top: 0;
    margin-bottom: 0;
  }

  ul,
  ol {
    padding: 0 1rem;
  }

  h1,
  h2,
  h3,
  h4,
  h5,
  h6 {
    line-height: 1.1;
  }

  code {
    background-color: rgba(#616161, 0.1);
    color: #616161;
  }

  pre {
    background: #0D0D0D;
    color: #FFF;
    font-family: 'JetBrainsMono', monospace;
    padding: 0.75rem 1rem;
    border-radius: 0.5rem;

    code {
      color: inherit;
      padding: 0;
      background: none;
      font-size: 0.8rem;
    }
  }

  mark {
    background-color: #FAF594;
  }

  img {
    max-width: 100%;
    height: auto;
  }

  hr {
    margin: 1rem 0;
  }

  blockquote {
    padding-left: 1rem;
    border-left: 1px solid rgba(#0D0D0D, 0.1);
  }

  hr {
    border: none;
    border-top: 1px solid rgba(#0D0D0D, 0.1);
    margin: 2rem 0;
  }

  ul[data-type="taskList"] {
    list-style: none;
    padding: 0;

    li {
      display: flex;
      align-items: center;

      > label {
        flex: 0 0 auto;
        margin-right: 0.5rem;
        user-select: none;
      }

      > div {
        flex: 1 1 auto;
      }
    }
  }
}
</style>
