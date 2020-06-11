#include <assert.h>
#include <stdbool.h>
#include <stdlib.h>

#include <catch2/catch.hpp>

#define LIBTRADER_TESTING
#include "libtrader.h"

TEST_CASE("linkedListNode testing", "[node]")
{
	struct LinkedListNode *prev = NULL, *node = NULL, *next = NULL;
	int *data = (int *)malloc(sizeof(int));
	*data = 42;

	/* test node creation with no prev and next */
	node = createLinkedListNode(NULL, NULL, NULL);
	REQUIRE(node);
	free(node);
	node = NULL;

	/* test node creation with prev and next */
	next = createLinkedListNode(NULL, NULL, NULL);
	prev = createLinkedListNode(NULL, NULL, NULL);
	node = createLinkedListNode(NULL, prev, next);

	REQUIRE((int *)(node->prev) == (int *)prev);
	REQUIRE((int *)(node->next) == (int *)next);

	free(node);
	free(prev);
	free(next);

	/* test node creation with data */
	node = createLinkedListNode(&data, NULL, NULL);
	REQUIRE(node->data);
	REQUIRE(node->data == &data);
	free(node);
	node = NULL;
	REQUIRE(data);
	REQUIRE(*data == 42);

	/* test node deletion */
	node = createLinkedListNode(NULL, NULL, NULL);
	node = destroyLinkedListNode(node);
	REQUIRE(node == NULL);

	/* test node deletion with data */
	node = createLinkedListNode(&data, NULL, NULL);
	node = destroyLinkedListNode(node);
	REQUIRE(data);
	REQUIRE(*data == 42);
}

TEST_CASE("linkedList testing", "[linkedlist]")
{
	LinkedList *list = NULL;
	struct LinkedListNode *prev = NULL, *node = NULL, *next = NULL;

	/* test empty linkedList creation */
	list = createLinkedList(0, NULL);
	REQUIRE(list);
	free(list);
	list = NULL;

	/* test correct size linkedList creation */
	prev = createLinkedListNode(NULL, NULL, NULL);
	node = createLinkedListNode(NULL, NULL, NULL);
	next = createLinkedListNode(NULL, NULL, NULL);
	prev->next = node;
	next->prev = node;
	node->prev = prev;
	node->next = next;

	list = createLinkedList(3, prev);

	REQUIRE(list->len == 3);
	REQUIRE(list->start == prev);
	REQUIRE(list->start->next == node);
	REQUIRE(list->start->next->prev == prev);
	REQUIRE(list->start->next->next == next);
	REQUIRE(list->start->next->next->prev == node);

	free(list);
	list = NULL;

	/* test wrong size linkedList creation */
	prev = createLinkedListNode(NULL, NULL, NULL);
	node = createLinkedListNode(NULL, NULL, NULL);
	prev->next = node;
	node->prev = prev;

	list = createLinkedList(3, prev);
	REQUIRE(list == NULL);
	REQUIRE(prev);
	REQUIRE(node);
	free(prev);
	free(node);
	node = NULL;
	prev = NULL;

	/* test wrong backward connection of nodes' linkedlist creation */
	prev = createLinkedListNode(NULL, NULL, NULL);
	node = createLinkedListNode(NULL, NULL, NULL);
	next = createLinkedListNode(NULL, NULL, NULL);
	prev->next = node;
	node->next = next;

	list = createLinkedList(3, prev);
	REQUIRE(list == NULL);
	REQUIRE(prev);
	REQUIRE(node);
	REQUIRE(next);
	free(prev);
	free(node);
	free(next);
	prev = NULL;
	node = NULL;
	next = NULL;

	/* test appendication of node to a list */
	prev = createLinkedListNode(NULL, NULL, NULL);
	node = createLinkedListNode(NULL, NULL, NULL);
	next = createLinkedListNode(NULL, NULL, NULL);
	prev->next = node;
	node->prev = prev;

	list = createLinkedList(2, prev);
	linkedList_add(list, next, list->len);
	REQUIRE(list->len == 3);
	REQUIRE(prev);
	REQUIRE(node);
	REQUIRE(next);
	REQUIRE(list->start == prev);
	REQUIRE(list->start->next == node);
	REQUIRE(list->start->next->prev == prev);
	REQUIRE(list->start->next->next == next);
	REQUIRE(list->start->next->next->prev == node);
	free(prev);
	free(node);
	free(next);
	prev = NULL;
	node = NULL;
	next = NULL;

	/* test retrieval of position in linkedlist */
	prev = createLinkedListNode(NULL, NULL, NULL);
	node = createLinkedListNode(NULL, NULL, NULL);
	next = createLinkedListNode(NULL, NULL, NULL);
	prev->next = node;
	node->prev = prev;
	node->next = next;
	next->prev = node;

	list = createLinkedList(3, prev);
	REQUIRE(linkedList_getPos(list, prev) == 0);
	REQUIRE(linkedList_getPos(list, node) == 1);
	REQUIRE(linkedList_getPos(list, next) == 2);
	free(prev);
	free(node);
	free(next);
	prev = NULL;
	node = NULL;
	next = NULL;

	/* test retrieval of node in linkedlist */
	prev = createLinkedListNode(NULL, NULL, NULL);
	node = createLinkedListNode(NULL, NULL, NULL);
	next = createLinkedListNode(NULL, NULL, NULL);
	prev->next = node;
	node->prev = prev;
	node->next = next;
	next->prev = node;
	list = createLinkedList(3, prev);
	REQUIRE(prev);
	REQUIRE(node);
	REQUIRE(next);
	REQUIRE(linkedList_getNode(list, 0) == prev);
	REQUIRE(linkedList_getNode(list, 1) == node);
	REQUIRE(linkedList_getNode(list, 2) == next);
	free(prev);
	free(node);
	free(next);
	prev = NULL;
	node = NULL;
	next = NULL;
}
