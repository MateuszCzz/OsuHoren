// turn off typescript type checking for component as it doesnt provide it
declare module 'vue-virtual-scroller' {
    import type { DefineComponent } from 'vue';
    export const RecycleScroller: any;
    export const DynamicScroller: any;
    export const DynamicScrollerItem: any;
}
