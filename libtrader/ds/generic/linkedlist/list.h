#ifndef DS_GENERIC_LINKED_LIST_H
#define DS_GENERIC_LINKED_LIST_H

#include <stdbool.h>
#include <stddef.h>

struct LinkedListNode {
	void *data;
	struct LinkedListNode *next;
	struct LinkedListNode *prev;
};
struct LinkedListNode *createLinkedListNode(void *data,
					    struct LinkedListNode *prev,
					    struct LinkedListNode *next);
bool destroyLinkedListNode(struct LinkedListNode *node);

typedef struct LinkedList {
	size_t len;
	struct LinkedListNode *start;
} LinkedList;
LinkedList *createLinkedList(size_t starting_size,
			     struct LinkedListNode *starting_node);
bool destroyLinkedList(LinkedList *list);
bool linkedList_verify(LinkedList *list);
void linkedList_add(LinkedList *list, struct LinkedListNode *node,
		    size_t position);
size_t linkedList_getPos(LinkedList *list, struct LinkedListNode *node);
struct LinkedListNode *linkedList_getNode(LinkedList *list, size_t position);

#endif
