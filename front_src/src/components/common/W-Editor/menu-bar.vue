<script setup>

</script>
<template>
  <div>
    <template v-for="(item, index) in items">
      <div class="divider" v-if="item.type === 'divider'" :key="`divider${index}`" />
      <MenuItem v-else :key="index" v-bind="item" />
    </template>
  </div>
</template>

<script>

import MenuItem from "./menu-item.vue";

export default {
  components: {
    MenuItem,
  },

  props: {
    editor: {
      type: Object,
      required: true,
    },
  },

  data() {
    return {
      items: [
        {
          icon: 'bold',
          title: '加粗',
          action: () => this.editor.chain().focus().toggleBold().run(),
          isActive: () => this.editor.isActive('bold'),
        },
        {
          icon: 'strikethrough',
          title: '删除线',
          action: () => this.editor.chain().focus().toggleStrike().run(),
          isActive: () => this.editor.isActive('strike'),
        },
        {
          icon: 'mark-pen-line',
          title: '高亮',
          action: () => this.editor.chain().focus().toggleHighlight().run(),
          isActive: () => this.editor.isActive('highlight'),
        },
        {
          icon: 'h-1',
          title: '一级标题',
          action: () => this.editor.chain().focus().toggleHeading({ level: 1 }).run(),
          isActive: () => this.editor.isActive('heading', { level: 1 }),
        },
        {
          icon: 'paragraph',
          title: '段落',
          action: () => this.editor.chain().focus().setParagraph().run(),
          isActive: () => this.editor.isActive('paragraph'),
        },
        {
          icon: 'format-clear',
          title: '清除格式',
          action: () => this.editor.chain()
              .focus()
              .clearNodes()
              .unsetAllMarks()
              .run(),
        },
        {
          type: 'divider',
        },
        {
          icon: 'arrow-go-back-line',
          title: '撤销',
          action: () => this.editor.chain().focus().undo().run(),
        },
        {
          icon: 'arrow-go-forward-line',
          title: '重做',
          action: () => this.editor.chain().focus().redo().run(),
        },
        {
          icon: 'space',
          title: '查看',
          action: () => console.log(this.editor.view.editable = false),
        },
      ],
    }
  },
}
</script>

<style  lang="sass">
.divider
  width: 2px
  height: 1.25rem
  background-color: rgba(#000, 0.1)
  margin-left: 0.5rem
  margin-right: 0.75rem
</style>
