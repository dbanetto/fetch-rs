PACKAGE=fetch
GOPATH=$(CURDIR)/.gopath
BASE=$(GOPATH)/src/$(PACKAGE)

.PHONY: build
build: get
	cd $(BASE) && go build -o bin/$(PACKAGE) cmd/fetcherd/main.go

$(BASE):
	mkdir -p $(dir $@)
	ln -sf $(CURDIR) $@

.PHONY: get
get: $(BASE)
	cd $(BASE) && go get -v -d ./

.PHONY: docker
docker: get
	CGO_ENABLED=0 GOOS=linux go build -ldflags "-X main.version=${version:-$(date -u +%Y%m%d.%H%M%S)}" -a -installsuffix cgo -v -o bin/fetch cmd/fetcherd/main.go


.PHONY: clean
clean:
	rm -rf $(BASE)
	rm -rf ./bin
