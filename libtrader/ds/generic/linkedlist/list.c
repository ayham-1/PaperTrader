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

struct LinkedListNode *destroyLinkedListNode(struct LinkedListNode *node)
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

	free(node);
	node = NULL;
	return node;
}

LinkedList *createLinkedList(size_t starting_size,
			     struct LinkedListNode *starting_node)
{
	LinkedList *list = (LinkedList *)malloc(sizeof(LinkedList));
	list->len = starting_size;
	list->start = starting_node;

	/* verify given nodes */
	if (!linkedList_verify(list)) {
#ifndef LIBTRADER_TESTING
		assert(0); /* TODO: add central error handling */
#else
		return NULL;
#endif
	}

	return list;
}

bool destroyLinkedList(LinkedList *list)
{
	assert(list);
	assert(linkedList_verify(list));

	struct LinkedListNode *ticker = list->start;
	while (list->len && list->start) {
		ticker = list->start->next;
		destroyLinkedListNode(list->start);
		list->start = ticker;
		list->len--;
	}
	return true;
}

bool linkedList_verify(LinkedList *list)
{
	assert(list);

	/* check if empty */
	if (list->len == 0 || list->start == NULL)
		return true;

	struct LinkedListNode *ticker = list->start;
	/* verify given nodes */
	for (size_t i = list->len - 1; (i != 0) && ticker; i--) {
		if (ticker->next ==
		    NULL) /* verify forward connection & size (indirectly) */
			return false;
		else if (ticker->next != NULL) {
			if (ticker->next->prev !=
			    ticker) /* verify backward connection*/
				return false;
			ticker = ticker->next;
		}
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
	if (!node) {
		struct LinkedListNode *last_node =
			linkedList_getNode(list, position - 1);
		last_node->next = new_node;
		new_node->prev = last_node;
	} else if (node && node->next) {
		node->next->prev = new_node;
		node->next = new_node;
		new_node->prev = node;
	} else {
		node->next = new_node;
		new_node->prev = node;
	}
	list->len++;

	assert(linkedList_verify(list));
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
	assert(linkedList_verify(list));
	assert(position || position == 0);

	/* no need to search for node if it is the start */
	if (position == 0)
		return list->start;

	/* search for the node */
	struct LinkedListNode *ticker = list->start;
	while (position) {
		if (ticker == NULL)
			return NULL;
		ticker = ticker->next;
		position--;
	}
	return ticker;
}
