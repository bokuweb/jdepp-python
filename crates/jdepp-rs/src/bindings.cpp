#include <algorithm>
#include <atomic>
#include <chrono>
#include <fstream>
#include <mutex>
#include <sstream>
#include <thread>
#include <vector>
#include <iostream>
#include <sstream>
#define OPTPARSE_IMPLEMENTATION
#include "../../../jdepp/optparse.h"
#include "../../../jdepp/pdep.h"
#include "../../../jdepp/io-util.hh"

#ifndef NUM_POS_FIELD
#define NUM_POS_FIELD 4
#endif

class Jdepp
{
public:
    const uint32_t kMaxChars = IOBUF_SIZE / 4; // Usually UTF-8 Japanese character is 3-bytes. div by 4 is convervative estimate.
    Jdepp() {}
    Jdepp(const std::string &model_path)
        : _model_path(model_path)
    {
        load_model(_model_path);
    }

    void set_threads(uint32_t nthreads)
    {
        _nthreads = nthreads;
    }

    bool load_model(const std::string &model_path)
    {

        //
        // Curently we must set all parameters in options as (argc, argv), then construct parser.
        //

        // std::cout << "model_path " << model_path << "\n";
        _argv_str.push_back("pyjdepp");
        //_argv_str.push_back("--verbose");
        //_argv_str.push_back("10");
        _argv_str.push_back("-m");
        _argv_str.push_back(model_path);

        // TODO: args for learner, chunker, depend

        setup_argv();

        pdep::option options(int(_argv.size()), _argv.data(),
                             int(_learner_argv.size()), _learner_argv.data(),
                             int(_chunk_argv.size()), _chunk_argv.data(),
                             int(_depend_argv.size()), _depend_argv.data());

        if (_parser && _parser->model_loaded())
        {
            // discard previous model&instance.
            delete _parser;
        }

        _parser = new pdep::parser(options);
        if (!_parser->load_model())
        { // use setting in argv for model path
          // py::print("Model load failed:", model_path);
        }

        return _parser->model_loaded();
    }

    bool model_loaded() const
    {
        return (_parser && _parser->model_loaded());
    }

    // for internal debugging
    void run()
    {
        _parser->run();
    }

private:
    void setup_argv()
    {
        _argv.clear();
        for (auto &v : _argv_str)
        {
            _argv.push_back(const_cast<char *>(v.c_str()));
        }
        _argv.push_back(nullptr); // must add 'nullptr' at the end, otherwise out-of-bounds access will happen in optparse

        for (auto &v : _learner_argv_str)
        {
            _learner_argv.push_back(const_cast<char *>(v.c_str()));
        }
        _learner_argv.push_back(nullptr);

        for (auto &v : _chunk_argv_str)
        {
            _chunk_argv.push_back(const_cast<char *>(v.c_str()));
        }
        _chunk_argv.push_back(nullptr);

        for (auto &v : _depend_argv_str)
        {
            _depend_argv.push_back(const_cast<char *>(v.c_str()));
        }
        _depend_argv.push_back(nullptr);
    }

    uint32_t _nthreads{0}; // 0 = use all cores
    std::string _model_path;
    pdep::parser *_parser{nullptr};

    std::vector<char *> _argv;
    std::vector<std::string> _argv_str;

    std::vector<char *> _learner_argv;
    std::vector<std::string> _learner_argv_str;

    std::vector<char *> _chunk_argv;
    std::vector<std::string> _chunk_argv_str;

    std::vector<char *> _depend_argv;
    std::vector<std::string> _depend_argv_str;
};

extern "C"
{
    Jdepp *jdepp_create(const char *model_path)
    {
        return new Jdepp(std::string(model_path));
    }

    void jdepp_destroy(Jdepp *instance)
    {
        delete instance;
    }

    bool jdepp_model_loaded(const Jdepp *instance)
    {
        return instance->model_loaded();
    }
}