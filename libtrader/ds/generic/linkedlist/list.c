#include <assert.h>
#include <stdlib.h>

#include "list.h"

struct LinkedListNode *createLinkedListNode(void *data,
					    struct LinkedListNode *prev,
					    struct LinkedListNode *next)
{
	struct LinkedListNode *node =
		(struct LinkedListNode *)malloc(sizeof(struct LinkedListNode));
	node->data = data;
	node->prev = prev;
	node->next = next;
	return node;
}

bool destroyLinkedListNode(struct LinkedListNode *node)
{
	assert(node);

	/* connect nodes */
	if (node->prev && node->next) {
		node->prev->next = node->next;
		node->next->prev = node->prev;
	} else if (node->prev)
		node->prev->next = NULL;
	else if (node->next)
		node->next->prev = NULL;
	else
		return false;

	free(node);
	return true;
}

LinkedList *createLinkedList(size_t starting_size,
			     struct LinkedListNode *starting_node)
{
	LinkedList *list = (LinkedList *)malloc(sizeof(LinkedList));
	list->len = starting_size;
	list->start = starting_node;

	/* verify given nodes */
	if (!linkedList_verify(list))
		exit(1); /* TODO: add central error handling */

	return list;
}

bool destroyLinkedList(LinkedList *list)
{
	assert(list);
	linkedList_verify(list);

	struct LinkedListNode *ticker = list->start;
	while (list->len && list->start) {
		ticker = list->start->next;
		if (!destroyLinkedListNode(list->start))
			return false;
		list->start = ticker;
		list->len--;
	}
	return true;
}

bool linkedList_verify(LinkedList *list)
{
	assert(list);
	/* verify given nodes */
	struct LinkedListNode *ticker = list->start;
	assert(ticker);
	ticker = ticker->next;
	for (size_t i = 1; i < list->len; i++) {
		if (ticker && ticker->next)
			ticker = ticker->next;
		else
			return false;
	}
	return true;
}

void linkedList_add(LinkedList *list, struct LinkedListNode *new_node,
		    size_t position)
{
	assert(list);
	assert(linkedList_verify(list));
	assert(new_node);
	assert(position || position == 0);
	assert(list->len >= position);

	struct LinkedListNode *node = linkedList_getNode(list, position);
	assert(node);
	if (node->next) {
		node->next->prev = new_node;
		node->next = new_node;
	} else
		node->next = new_node;
	list->len++;
}

size_t linkedList_getPos(LinkedList *list, struct LinkedListNode *node)
{
	assert(list);
	assert(node);

	struct LinkedListNode *ticker = list->start;
	for (size_t i = 0; i < list->len; i++)
		if (ticker == node)
			return i;
		else
			ticker = ticker->next;

	return -1;
}

struct LinkedListNode *linkedList_getNode(LinkedList *list, size_t position)
{
	assert(list);
	assert(position || position == 0);

	struct LinkedListNode *ticker = list->start;
	while (position) {
		ticker = ticker->next;
		position--;
	}
	return ticker;
}
