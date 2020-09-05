

class TestCase:
    def __init__(self, name):
        self.name = name

    def set_up(self):
        pass

    def run(self):
        self.set_up()
        method = getattr(self, self.name)
        method()


class WasRun(TestCase):
    def __init__(self, name):
        self.wasRun = None
        self.wasSetUp = None
        TestCase.__init__(self, name)

    def set_up(self):
        self.wasRun
        self.wasSetUp = 1

    def test_method(self):
        self.wasRun = 1


class TestCaseTest(TestCase):
    def set_up(self):
        self.test = WasRun("test_method")

    def test_running(self):
        self.test.run()
        assert self.test.wasRun

    def test_setup(self):
        self.test.run()
        assert self.test.wasSetUp


TestCaseTest("test_running").run()
TestCaseTest("test_setup").run()
