#ifndef DS_GENERIC_LINKED_LIST_H
#define DS_GENERIC_LINKED_LIST_H

typedef struct LinkedListNode {
	void *data;
	LinkedListNode *next;
	LinkedListNode *prev;
} LinkedListNode;
LinkedListNode *createLinkedListNode(void *data, LinkedListNode *next,
				     LinkedlistNode *prev);
bool destroyLinkedListNode(LinkedListNode *node);

typedef struct LinkedList {
	size_t len;
	LinkedListNode *start;
	LinkedListNode *current;
} LinkedList;
LinkedList *createLinkedList(size_t starting_size,
			     LinkedListNode *starting_node);
bool destroyLinkedList(LinkedList *list);
bool linkedList_add(LinkedList *list, LinkedListNode *node, size_t position);
bool linkedList_remove(LinkedList *list, size_t position);
size_t linkedlist_getPos(LinkedList *list, LinkedListNode *node);
size_t linkedlist_getNode(LinkedList *list, size_t position);

#endif
